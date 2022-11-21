#![feature(map_try_insert)]
#![no_std]

use gstd::prelude::*;

mod contract {
    use crate::*;

    #[derive(Default)]
    pub struct Contract(pub bool);

    impl Contract {
        pub fn set(&mut self, b: bool) {
            self.0 = b
        }
    }
}
use contract::Contract;

mod codec {
    use crate::*;
    use parity_scale_codec::{Decode, Encode};
    use scale_info::TypeInfo;

    pub mod action {
        use super::*;
        #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
        pub enum Action {
            Set(bool),
        }

        #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
        pub enum Event {
            SetTo(bool),
        }
    }

    pub mod query {
        use super::*;
        #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
        pub enum Query {
            Status,
        }

        #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
        pub enum State {
            StatusOf(bool),
        }
    }
}
use codec::{
    action::{Action, Event},
    query::{Query, State},
};

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
        Action::Set(b) => {
            state.set(b);
            gstd::msg::reply(Event::SetTo(state.0), 0).expect("failed to reply");
        }
    }
}

#[no_mangle]
unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let state = STATE.as_ref().expect("failed to get contract state");
    let query: Query = gstd::msg::load().expect("failed to load query");
    let result = match query {
        Query::Status => State::StatusOf(state.0),
    };
    gstd::util::to_leak_ptr(result.encode())
}

gstd::metadata! {
    title: "github.com/btwiuse/gboard",
    handle:
        input: Action,
        output: Event,
    state:
        input: Query,
        output: State,
}
