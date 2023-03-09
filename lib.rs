#![no_std]

use gstd::prelude::*;
use io::*;

// contract state
static mut STATE: Option<Contract> = None;

// what this line says is "here is a C function written in Rust"
#[no_mangle]
unsafe extern "C" fn init() {
    STATE = Some(Contract::default());
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let state = STATE.as_mut().expect("failed to get state as mut");
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
    let state = unsafe { STATE.as_ref().expect("failed to get contract state") };
    gstd::msg::reply(state.0.clone(), 0).expect("Failed to share state");
}

#[no_mangle]
extern "C" fn metahash() {
    let metahash: [u8; 32] = include!(".metahash");
    gstd::msg::reply(metahash, 0).expect("Failed to share metahash");
}
