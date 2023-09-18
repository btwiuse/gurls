#![no_std]

use gstd::prelude::*;
use io::*;

// contract state
static mut STATE: Option<Contract> = None;

// what this line says is "here is a C function written in Rust"
#[no_mangle]
extern "C" fn init() {
    unsafe { STATE = Some(Contract::default()) };
}

#[no_mangle]
extern "C" fn handle() {
    let state = unsafe { STATE.as_mut().expect("failed to get state as mut") };
    let action: Action = gstd::msg::load().expect("failed to load action");
    match action {
        Action::AddUrl { code, url } => {
            state.add_url(code.clone(), url.clone());
            gstd::msg::reply(Event::Added { code, url }, 0).expect("failed to reply");
        }
    }
}

#[no_mangle]
extern "C" fn state() {
    let query = gstd::msg::load().expect("failed to load query");
    let state = unsafe { STATE.as_ref().expect("failed to get contract state") };
    match query {
        Query::All => {
            gstd::msg::reply(Reply::All(state.clone()), 0).expect("Failed to share state");
        }
        Query::Code(code) => {
            gstd::msg::reply(Reply::Url(state.get_url(code)), 0).expect("Failed to share state");
        }
    }
}
