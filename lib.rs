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
    let reply = match query {
        Query::All => Reply::All(state.clone()),
        Query::Code(code) => Reply::Url(state.get_url(code)),
        Query::Whoami => Reply::Whoami(gstd::msg::source()), // all zero addr
        Query::BlockNumber => Reply::BlockNumber(gstd::exec::block_height()),
        Query::BlockTimestamp => Reply::BlockTimestamp(gstd::exec::block_timestamp()),
        Query::ProgramId => Reply::ProgramId(gstd::exec::program_id()),
        Query::MessageId => Reply::MessageId(gstd::msg::id()), // all zero id
    };
    gstd::msg::reply(reply, 0).expect("Failed to share state");
}
