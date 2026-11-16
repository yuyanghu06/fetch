//! Keep a wheel gesture in the deepest scroll area where it began.
use std::{cell::RefCell,time::{Duration,Instant}};
#[derive(Default)]
struct Gesture {path:Vec<u64>,stack:Vec<u64>,last:Option<Instant>}
thread_local! {static GESTURE:RefCell<Gesture>=RefCell::new(Gesture::default());}
/// Called once by the root wrapper for each wheel event. Native touch/momentum
/// phases distinguish a fresh finger gesture from its inertial continuation.
pub(super) fn begin()->bool{
    begin_phase(native_phase(),Instant::now())
}
fn begin_phase(phase:u32,now:Instant)->bool{
    GESTURE.with(|value|{
        let mut state=value.borrow_mut();
        let fresh=phase&4!=0||(phase&2==0&&state.last.is_none_or(|last|now.duration_since(last)>Duration::from_millis(180)));
        if fresh{state.path.clear();}
        state.stack.clear();state.last=Some(now);fresh
    })
}
pub(super) fn enter(id:u64,over:bool)->bool{GESTURE.with(|value|{
    let mut state=value.borrow_mut();
    if (state.path.is_empty()&&!over)||(!state.path.is_empty()&&!state.path.contains(&id)){return false;}
    state.stack.push(id);true
})}
pub(super) fn owner(id:u64)->bool{GESTURE.with(|value|value.borrow().path.last()==Some(&id))}
pub(super) fn leave(id:u64){GESTURE.with(|value|{
    let mut state=value.borrow_mut();if state.path.is_empty(){state.path=state.stack.clone();}
    debug_assert_eq!(state.stack.pop(),Some(id));
})}
#[cfg(target_os="macos")]
fn native_phase()->u32{unsafe extern "C"{fn fetch_take_scroll_phase()->u32;}unsafe{fetch_take_scroll_phase()}}
#[cfg(not(target_os="macos"))]
fn native_phase()->u32{0}
pub(super) fn reset(){GESTURE.with(|state|*state.borrow_mut()=Gesture::default());}
#[cfg(test)]
pub(super) fn new_test_gesture(){reset();}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn physical_gesture_and_momentum_keep_owner_until_new_finger_begin(){
        reset();let now=Instant::now();assert!(begin_phase(7,now));
        assert!(enter(1,true));leave(1);assert!(owner(1));
        assert!(!begin_phase(3,now+Duration::from_secs(3)));
        assert!(enter(1,true));assert!(!enter(2,true));leave(1);
        assert!(!begin_phase(3,now+Duration::from_secs(4)));assert!(owner(1));
        assert!(begin_phase(7,now+Duration::from_secs(5)));
        assert!(enter(1,true));assert!(enter(2,true));leave(2);leave(1);assert!(owner(2));
        reset();
    }
}
