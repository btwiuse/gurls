#![no_std]

use gstd::prelude::*;

use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Default, Encode, Decode, TypeInfo)]
pub struct Contract(pub BTreeMap<String, String>);

impl Contract {
    pub fn add_url(&mut self, code: String, url: String) {
        if self.0.contains_key(&code) {
            panic!("failed to add url: code exists");
        } else {
            self.0.insert(code, url);
        }
    }
    pub fn get_url(&self, code: String) -> Option<String> {
        self.0.get(&code).cloned()
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Action {
    AddUrl { code: String, url: String },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Event {
    Added { code: String, url: String },
}

use gmeta::{InOut, Metadata};

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    // type State = BTreeMap<String, String>;
    type State = Contract;
}
