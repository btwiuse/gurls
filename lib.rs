#![feature(map_try_insert)]
#![no_std]

use gstd::prelude::*;

mod codec {
    use crate::*;
    use parity_scale_codec::{Decode, Encode};
    use scale_info::TypeInfo;

    pub mod action {
        use super::*;
        #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
        pub enum Action {
            AddMsg(String),
        }

        #[derive(Debug, Clone, Encode, Decode, TypeInfo)]
        pub enum Event {
            AddedMsg { msg: String, by: gstd::ActorId },
        }
    }
}
use codec::action::{Action, Event};

#[no_mangle]
extern "C" fn handle() {
    let action: Action = gstd::msg::load().expect("failed to load action");
    match action {
        Action::AddMsg(msg) => {
            let by = gstd::exec::origin();
            gstd::msg::reply(Event::AddedMsg { msg, by }, 0).expect("failed to reply");
        }
    }
}

gstd::metadata! {
    title: "groupchat",
    handle:
        input: Action,
        output: Event,
}
