#![no_std]

use gstd::prelude::*;
use io::Contract;

// contract state
static mut STATE: Option<Contract> = None;

// what this line says is "here is a C function written in Rust"
#[no_mangle]
extern "C" fn init() {
    unsafe { STATE = Some(Contract::default()) }
}

#[no_mangle]
extern "C" fn handle() {
    // use io::{Action, Event};
    type Handle = <Contract as gmeta::Metadata>::Handle;
    type Action = <Handle as gmeta::Types>::Input;
    type Event = <Handle as gmeta::Types>::Output;

    let state: &mut Contract = unsafe { STATE.as_mut().expect("failed to get state as mut") };
    let action: Action = gstd::msg::load().expect("failed to load action");
    let event: Event = match action {
        Action::AddUrl { code, url } => {
            state.add_url(code.clone(), url.clone());
            Event::Added { code, url }
        }
        Action::SendValue { to, value } => {
            gstd::msg::send_bytes(to, [], value).expect("failed to send value");
            Event::SentValue { to, value }
        }
        Action::SendValueTwice { to, value } => {
            gstd::msg::send(
                gstd::exec::program_id(),
                Action::SendValue { to, value },
                value,
            )
            .expect("failed to send value 1");
            gstd::msg::send(
                gstd::exec::program_id(),
                Action::SendValue { to, value },
                value,
            )
            .expect("failed to send value 2");
            Event::SentValueTwice { to, value }
        }
    };
    gstd::msg::reply(event, 0).expect("failed to reply");
}

#[no_mangle]
extern "C" fn state() {
    // use io::{Query, Reply};
    type State = <Contract as gmeta::Metadata>::State;
    type Query = <State as gmeta::Types>::Input;
    type Reply = <State as gmeta::Types>::Output;

    let state: &Contract = unsafe { STATE.as_ref().expect("failed to get contract state") };
    let query: Query = gstd::msg::load().expect("failed to load query");
    let reply: Reply = match query {
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
