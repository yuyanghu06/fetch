//! The native picker only returns a path. Reading it still requires Discover.

#[cfg(target_os="macos")]
use std::{ffi::{CStr,c_char,c_void},future::Future};
#[cfg(target_os="macos")]
use tokio::sync::oneshot;

#[cfg(target_os="macos")]
type PickerResult=Result<Option<String>,String>;

#[cfg(target_os="macos")]
unsafe extern "C" {
    fn fetch_choose_config_file(callback:unsafe extern "C" fn(*mut c_void,*const c_char,i32),context:*mut c_void);
    fn fetch_cancel_config_file_picker();
}

#[cfg(target_os="macos")]
unsafe extern "C" fn completed(context:*mut c_void,path:*const c_char,status:i32) {
    // The native bridge calls back once, including cancellation and unavailable UI.
    // Reclaim the sender here even if the receiving task has already gone away.
    let sender=unsafe {Box::from_raw(context.cast::<oneshot::Sender<PickerResult>>())};
    let result=match status {
        1=>Ok(None),
        0 if !path.is_null()=>{
            // The pointer belongs to AppKit and is valid only during this callback.
            match unsafe {CStr::from_ptr(path)}.to_str() {
                Ok(path) if std::path::Path::new(path).is_absolute()=>Ok(Some(path.to_owned())),
                _=>Err("This file path cannot be used. Choose another file or type an absolute path.".into()),
            }
        }
        _=>Err("The file picker could not open. Try again or type an absolute configuration path.".into()),
    };
    let _=sender.send(result);
}

#[cfg(target_os="macos")]
pub fn choose_config_file()->impl Future<Output=PickerResult>+Send {
    let receiver={
        let (sender,receiver)=oneshot::channel::<PickerResult>();
        let context=Box::into_raw(Box::new(sender)).cast::<c_void>();
        // This only queues AppKit work; it neither waits nor reads the selected file.
        unsafe {fetch_choose_config_file(completed,context);}
        receiver
    };
    async move {receiver.await.unwrap_or_else(|_|Err("The file picker closed before returning a selection. Try again.".into()))}
}

#[cfg(target_os="macos")]
pub fn cancel() {
    // Native cancellation is queued after any queued presentation request.
    unsafe {fetch_cancel_config_file_picker();}
}

#[cfg(not(target_os="macos"))]
pub async fn choose_config_file()->Result<Option<String>,String> {
    Err("The native file picker is available on macOS. Type an absolute configuration path.".into())
}

#[cfg(not(target_os="macos"))]
pub fn cancel() {}
