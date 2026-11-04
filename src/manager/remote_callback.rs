//! One attempt-local browser return address; this listener exposes no product APIs.
use super::{Manager,ManagementError,now};
use axum::{Router,extract::{Request,State},http::{StatusCode,header,HeaderValue},response::{IntoResponse,Response},routing::get};
use std::{sync::{Arc,Weak,atomic::Ordering},time::Duration};
use tokio::{net::TcpListener,sync::{Notify,watch},task::JoinHandle};

pub(super) struct Pending {listener:TcpListener,redirect:String}
pub(super) struct Close(watch::Sender<bool>);
impl Close {pub(super) fn close(&self){self.0.send_replace(true);}}

#[derive(Clone)]
struct CallbackState {manager:Weak<Manager>,expected_state:String}

impl Pending {
    pub(super) async fn bind()->Result<Self,ManagementError>{
        let unavailable=||ManagementError::new(500,"AUTHENTICATION_FAILED","Fetch could not open its local sign-in callback listener.");
        let listener=TcpListener::bind((std::net::Ipv4Addr::LOCALHOST,0)).await.map_err(|_|unavailable())?;
        let address=listener.local_addr().map_err(|_|unavailable())?;
        Ok(Self{listener,redirect:format!("http://127.0.0.1:{}/oauth/provider/callback",address.port())})
    }
    pub(super) fn redirect_uri(&self)->String{self.redirect.clone()}
    pub(super) fn start(self,manager:Weak<Manager>,expected_state:String,expires:u64,stop:Arc<Notify>)->(Close,JoinHandle<()>) {
        let(close,mut closing)=watch::channel(false);
        let router=Router::new().route("/oauth/provider/callback",get(callback))
            .with_state(CallbackState{manager:manager.clone(),expected_state:expected_state.clone()});
        let task=tokio::spawn(async move{
            let shutdown=async move{
                let quit=stop.notified();tokio::pin!(quit);quit.as_mut().enable();
                if *closing.borrow()||manager.upgrade().is_none_or(|manager|manager.stopping.load(Ordering::SeqCst)){return;}
                tokio::select!{
                    _=closing.changed()=>{},
                    _=&mut quit=>{},
                    _=tokio::time::sleep(Duration::from_secs(expires.saturating_sub(now())))=>{
                        if let Some(manager)=manager.upgrade(){
                            let flow=manager.flows.lock().await.remove(&expected_state);
                            if let Some(flow)=flow{
                                flow.diagnostics.text("oauth browser","The sign-in page expired before a completed callback.");
                                manager.end_auth_failure(&flow.server,&flow.attempt,"AUTHENTICATION_TIMEOUT","The browser login expired. Start a fresh explicit login.").await;
                            }
                        }
                    }
                }
            };
            if axum::serve(self.listener,router).with_graceful_shutdown(shutdown).await.is_err(){
                crate::diagnostics::application_failure("AUTHENTICATION_CALLBACK_LISTENER_FAILED");
            }
        });
        (Close(close),task)
    }
}

async fn callback(State(state):State<CallbackState>,request:Request)->Response{
    let manager=state.manager.upgrade().ok_or_else(||(StatusCode::SERVICE_UNAVAILABLE,"Fetch is no longer running.").into_response());
    let mut response=crate::api::runtime::complete_provider_callback(manager,request,Some(&state.expected_state)).await;
    response.headers_mut().insert(header::CACHE_CONTROL,HeaderValue::from_static("no-store"));
    response
}
