#![feature(map_try_insert)]
#![feature(trait_alias)]
#![no_std]

use gstd::prelude::*;

pub mod contract {
    use super::{BTreeMap, Default, String};

    #[derive(Default)]
    pub struct Contract {
        pub urls: BTreeMap<String, String>,
    }

    impl Contract {
        pub fn set_url(&mut self, key: String, value: String) {
            self.urls
                .try_insert(key, value)
                .expect("failed to insert: key already exists");
        }

        pub fn get_url(&self, key: String) -> Option<String> {
            self.urls.get(&key).cloned()
        }
    }
}

mod state {
    use super::contract::Contract;

    pub static mut STATE: Option<Contract> = None;
}

pub mod codec {
    use super::String;

    use parity_scale_codec::{Decode, Encode};
    use scale_info::TypeInfo;

    #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
    pub enum Action {
        SetUrl(String, String),
    }

    #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
    pub enum Event {
        SetUrl(String, String),
    }

    #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
    pub enum Query {
        GetUrl(String),
    }

    #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
    pub enum State {
        Url(Option<String>),
    }
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    state::STATE = Some(contract::Contract::default());
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    let state = state::STATE.as_mut().expect("failed to get state as mut");
    let action: codec::Action = gstd::msg::load().expect("failed to load action");
    match action {
        codec::Action::SetUrl(key, value) => {
            state.set_url(key.clone(), value.clone());
            gstd::msg::reply(codec::Event::SetUrl(key, value), 0).expect("failed to set url");
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn meta_state() -> *mut [i32; 2] {
    let state = state::STATE.as_ref().expect("failed to get contract state");
    let query: Query = gstd::msg::load().expect("failed to decode input argument");
    let encoded = match query {
        Query::GetUrl(key) => {
            let value = state.get_url(key);
            State::Url(value)
        }
    }
    .encode();
    gstd::util::to_leak_ptr(encoded)
}

use codec::*;
gstd::metadata! {
    title: "GURLS",
    handle:
        input: Action,
        output: Event,
    state:
        input: Query,
        output: State,
}
