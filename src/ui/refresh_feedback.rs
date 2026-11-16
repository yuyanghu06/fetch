//! Shared refresh feedback. Results apply immediately; only the indicator waits.
use iced::Task;
use std::time::Duration;

#[derive(Default)]
pub(super) struct RefreshFeedback {
    pub(super) pending: Option<u64>,
    pub(super) minimum: Option<u64>,
    pub(super) generation: u64,
}
impl RefreshFeedback {
    pub(super) fn running(&self) -> bool {
        self.pending.is_some() || self.minimum.is_some()
    }
    pub(super) fn begin(&mut self, sequence: u64, minimum: Duration) -> Task<u64> {
        self.pending = Some(sequence);
        self.minimum = Some(sequence);
        self.generation = sequence;
        Task::perform(async move {
            tokio::time::sleep(minimum).await;
            sequence
        }, |sequence| sequence)
    }
    pub(super) fn finish(&mut self, sequence: u64) {
        if self.pending == Some(sequence) { self.pending = None; }
    }
    pub(super) fn elapsed(&mut self, sequence: u64) {
        if self.minimum == Some(sequence) { self.minimum = None; }
    }
}
