#![feature(map_try_insert)]
#![no_std]

use gstd::prelude::*;

mod contract {
    pub struct Contract(pub [bool; 401]);

    impl Contract {
        pub fn dump_all(&self) -> [bool; 401] {
            self.0
        }
        pub fn pixel(&self, n: u16) -> bool {
            self.0[n as usize]
        }
        pub fn set(&mut self, b: bool) {
            self.set_pixel(0, b)
        }
        pub fn toggle(&mut self) {
            self.set(!self.pixel(0))
        }
        pub fn set_pixel(&mut self, n: u16, b: bool) {
            self.0[n as usize] = b
        }
        pub fn toggle_pixel(&mut self, n: u16) {
            self.set_pixel(n, !self.pixel(n))
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
            Toggle,
            Set(bool),
            TogglePixel(u16),
            SetPixel(u16, bool),
        }

        #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
        pub enum Event {
            SetTo(u16, bool),
        }
    }

    pub mod query {
        use super::*;
        #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
        pub enum Query {
            Status,
            Pixel(u16),
            DumpAll,
        }

        #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
        pub enum State {
            StatusOf(bool),
            All([bool; 401]),
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
    STATE = Some(Contract([false; 401]));
}

#[no_mangle]
unsafe extern "C" fn handle() {
    let state = STATE.as_mut().expect("failed to get state as mut");
    let action: Action = gstd::msg::load().expect("failed to load action");
    match action {
        Action::Set(b) => {
            state.set(b);
            gstd::msg::reply(Event::SetTo(0, state.pixel(0)), 0).expect("failed to reply");
        }
        Action::Toggle => {
            state.toggle();
            gstd::msg::reply(Event::SetTo(0, state.pixel(0)), 0).expect("failed to reply");
        }
        Action::SetPixel(n, b) => {
            state.set_pixel(n, b);
            gstd::msg::reply(Event::SetTo(n, state.pixel(n)), 0).expect("failed to reply");
        }
        Action::TogglePixel(n) => {
            state.toggle_pixel(n);
            gstd::msg::reply(Event::SetTo(n, state.pixel(n)), 0).expect("failed to reply");
        }
    }
}

#[no_mangle]
unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let state = STATE.as_ref().expect("failed to get contract state");
    let query: Query = gstd::msg::load().expect("failed to load query");
    let result = match query {
        Query::Status => State::StatusOf(state.pixel(0)),
        Query::Pixel(n) => State::StatusOf(state.pixel(n)),
        Query::DumpAll => State::All(state.dump_all()),
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
