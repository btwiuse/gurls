#![no_std]

use gmeta::{InOut, Metadata};
use gstd::collections::*;
use gstd::prelude::*;

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = ();
    type Handle = InOut<Action, Event>;
    type Others = ();
    type Reply = ();
    type Signal = ();
    type State = InOut<Query, Reply>;
}

#[derive(Clone, Default, Encode, Decode, TypeInfo)]
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

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Query {
    All,
    Code(String),
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Reply {
    All(Contract),
    Url(Option<String>),
}
