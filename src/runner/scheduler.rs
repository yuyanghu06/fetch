//! One bounded FIFO queue per logical server. Uncertain calls keep their slot.
use super::{CancelToken, ExecutionPolicy, RunError};
use serde_json::{Value, json};
use std::{collections::{BTreeMap, VecDeque}, sync::{Arc, Mutex}, time::Duration};
use tokio::{sync::{Notify, oneshot}, time::Instant};

const MAX_QUEUED: usize = 64;
const MAX_QUEUED_BYTES: usize = 16 * 1024 * 1024;
const QUEUE_WAIT: Duration = Duration::from_secs(300);

struct Waiting {
    id: u64,
    bytes: usize,
    expires: Instant,
    cancel: CancelToken,
    reply: oneshot::Sender<Result<(), RunError>>,
}
struct State {
    policy: ExecutionPolicy,
    next_id: u64,
    ready: bool,
    restarting: bool,
    active: BTreeMap<u64, bool>,
    waiting: VecDeque<Waiting>,
    waiting_bytes: usize,
}
pub struct Scheduler {
    state: Mutex<State>,
    changed: Notify,
}
impl Scheduler {
    pub fn new(policy: ExecutionPolicy) -> Arc<Self> {
        Arc::new(Self { state: Mutex::new(State { policy, next_id: 0, ready: false, restarting: false, active: BTreeMap::new(), waiting: VecDeque::new(), waiting_bytes: 0 }), changed: Notify::new() })
    }
    pub fn ready(&self, ready: bool) {
        let mut state = self.state.lock().expect("scheduler lock");
        state.ready = ready;
        if !ready { Self::flush(&mut state, RunError::not_ready()); }
        Self::pump(&mut state);
        self.changed.notify_waiters();
    }
    pub fn update(&self, policy: ExecutionPolicy) {
        let mut state = self.state.lock().expect("scheduler lock");
        state.policy = policy;
        Self::pump(&mut state);
        self.changed.notify_waiters();
    }
    pub fn timeout(&self, name: Option<&str>) -> Duration {
        let state = self.state.lock().expect("scheduler lock");
        Duration::from_millis(name.and_then(|name| state.policy.tool_timeouts_ms.get(name).copied()).unwrap_or(state.policy.default_timeout_ms))
    }
    pub async fn admit(self: &Arc<Self>, bytes: usize, cancel: CancelToken) -> Result<Reservation, RunError> {
        let (reply, receiver) = oneshot::channel();
        let (id, expires) = {
            let mut state = self.state.lock().expect("scheduler lock");
            if state.restarting { return Err(RunError::busy()); }
            if !state.ready { return Err(RunError::not_ready()); }
            Self::pump(&mut state);
            if !state.waiting.is_empty() || state.active.len() >= state.policy.max_concurrent_calls {
                if state.waiting.len() >= MAX_QUEUED || bytes > MAX_QUEUED_BYTES.saturating_sub(state.waiting_bytes) {
                    return Err(RunError::busy());
                }
            }
            state.next_id = state.next_id.checked_add(1).ok_or_else(RunError::busy)?;
            let id = state.next_id;
            let expires = Instant::now() + QUEUE_WAIT;
            state.waiting_bytes += bytes;
            state.waiting.push_back(Waiting { id, bytes, expires, cancel: cancel.clone(), reply });
            Self::pump(&mut state);
            (id, expires)
        };
        let mut guard = WaitingGuard { scheduler: self.clone(), id, armed: true };
        let result = tokio::select! {
            value = receiver => value.unwrap_or_else(|_| Err(RunError::not_ready())),
            _ = tokio::time::sleep_until(expires) => Err(RunError::queue_timeout()),
            _ = cancel.cancelled() => Err(RunError::cancelled(false)),
        };
        match result {
            Ok(()) => {
                guard.armed = false;
                if cancel.is_cancelled() { self.complete(id); return Err(RunError::cancelled(false)); }
                Ok(Reservation { scheduler: self.clone(), id, forwarded: std::sync::atomic::AtomicBool::new(false) })
            }
            Err(error) => Err(error),
        }
    }
    fn pump(state: &mut State) {
        while state.ready && !state.restarting && state.active.len() < state.policy.max_concurrent_calls {
            let Some(waiting) = state.waiting.pop_front() else { break; };
            state.waiting_bytes = state.waiting_bytes.saturating_sub(waiting.bytes);
            if waiting.expires <= Instant::now() || waiting.cancel.is_cancelled() || waiting.reply.is_closed() {
                let _ = waiting.reply.send(Err(RunError::queue_timeout()));
                continue;
            }
            state.active.insert(waiting.id, false);
            if waiting.reply.send(Ok(())).is_err() { state.active.remove(&waiting.id); }
        }
    }
    fn remove_waiting(&self, id: u64) {
        let mut state = self.state.lock().expect("scheduler lock");
        if let Some(index) = state.waiting.iter().position(|entry| entry.id == id) {
            let entry = state.waiting.remove(index).expect("located entry");
            state.waiting_bytes = state.waiting_bytes.saturating_sub(entry.bytes);
        } else if state.active.get(&id) == Some(&false) {
            // Admission won a race with dropped/expired waiter before any dispatch.
            state.active.remove(&id);
        }
        Self::pump(&mut state);
        self.changed.notify_waiters();
    }
    fn complete(&self, id: u64) {
        let mut state = self.state.lock().expect("scheduler lock");
        state.active.remove(&id);
        Self::pump(&mut state);
        self.changed.notify_waiters();
    }
    fn uncertain(&self, id: u64) {
        if let Some(value) = self.state.lock().expect("scheduler lock").active.get_mut(&id) { *value = true; }
        self.changed.notify_waiters();
    }
    fn flush(state: &mut State, error: RunError) {
        for entry in state.waiting.drain(..) { let _ = entry.reply.send(Err(error.clone())); }
        state.waiting_bytes = 0;
    }
    pub fn begin_restart(&self) -> Result<(), RunError> {
        let mut state = self.state.lock().expect("scheduler lock");
        if state.restarting { return Err(RunError::busy()); }
        state.restarting = true;
        Self::flush(&mut state, RunError::busy());
        self.changed.notify_waiters();
        Ok(())
    }
    pub fn reopen(&self) {
        let mut state = self.state.lock().expect("scheduler lock");
        state.restarting = false;
        Self::pump(&mut state);
        self.changed.notify_waiters();
    }
    /// Only confirmed child exit, or the selected explicit remote-force recovery,
    /// may clear unresolved reservations without a terminal upstream response.
    pub fn confirmed_reset(&self) -> usize {
        let mut state = self.state.lock().expect("scheduler lock");
        let count = state.active.len();
        state.active.clear();
        self.changed.notify_waiters();
        count
    }
    pub async fn drain(&self, budget: Duration) -> bool {
        let deadline = Instant::now() + budget;
        loop {
            let change = self.changed.notified();
            if self.state.lock().expect("scheduler lock").active.is_empty() { return true; }
            if tokio::time::timeout_at(deadline, change).await.is_err() { return false; }
        }
    }
    pub fn snapshot(&self) -> Value {
        let state = self.state.lock().expect("scheduler lock");
        json!({"active_calls":state.active.len(),"queued_calls":state.waiting.len(),"uncertain_calls":state.active.values().filter(|value| **value).count(),"max_concurrent_calls":state.policy.max_concurrent_calls,"max_queued_calls":MAX_QUEUED,"max_queued_bytes":MAX_QUEUED_BYTES,"queue_wait_timeout_ms":QUEUE_WAIT.as_millis() as u64})
    }
}
struct WaitingGuard { scheduler: Arc<Scheduler>, id: u64, armed: bool }
impl Drop for WaitingGuard { fn drop(&mut self) { if self.armed { self.scheduler.remove_waiting(self.id); } } }
pub struct Reservation { scheduler: Arc<Scheduler>, id: u64, forwarded: std::sync::atomic::AtomicBool }
impl Reservation {
    pub fn complete(self) { self.scheduler.complete(self.id); }
    pub fn mark_forwarded(&self) { self.forwarded.store(true, std::sync::atomic::Ordering::Release); }
    pub fn mark_uncertain(&self) { self.mark_forwarded(); self.scheduler.uncertain(self.id); }
}
impl Drop for Reservation {
    fn drop(&mut self) {
        if self.forwarded.load(std::sync::atomic::Ordering::Acquire) { self.scheduler.uncertain(self.id); }
        else { self.scheduler.complete(self.id); }
    }
}
