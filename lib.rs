#![feature(map_try_insert)]
#![no_std]

use gstd::prelude::*;

mod contract {
    use crate::*;

    #[derive(Default)]
    pub struct Contract(BTreeMap<String, String>);

    impl Contract {
        pub fn add_url(&mut self, code: String, url: String) {
            self.0
                .try_insert(code, url)
                .expect("failed to add url: code exists");
        }
        pub fn get_url(&self, code: String) -> Option<String> {
            self.0.get(&code).cloned()
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
            AddUrl { code: String, url: String },
        }

        #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
        pub enum Event {
            Added { code: String, url: String },
        }
    }

    pub mod query {
        use super::*;
        #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
        pub enum Query {
            Code(String),
        }

        #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
        pub enum State {
            MaybeUrl(Option<String>),
        }
    }
}
use codec::{
    action::{Action, Event},
    query::{Query, State},
};

static mut STATE: Option<Contract> = None;

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
unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let state = STATE.as_ref().expect("failed to get contract state");
    let query: Query = gstd::msg::load().expect("failed to load query");
    let result = match query {
        Query::Code(code) => State::MaybeUrl(state.get_url(code)),
    };
    gstd::util::to_leak_ptr(result.encode())
}

gstd::metadata! {
    title: "github.com/btwiuse/gurls",
    handle:
        input: Action,
        output: Event,
    state:
        input: Query,
        output: State,
}
