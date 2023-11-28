#![no_std]

use gmeta::{InOut, Metadata};
use gstd::collections::{btree_map::Entry, BTreeMap};
use gstd::prelude::*;
use gstd::ActorId;
use gstd::MessageId;

impl Metadata for Contract {
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
        match self.0.entry(code) {
            Entry::Vacant(v) => {
                v.insert(url);
            }
            Entry::Occupied(_) => {
                panic!("failed to add url: code exists")
            }
        }
    }
    pub fn get_url(&self, code: String) -> Option<String> {
        self.0.get(&code).cloned()
    }
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Action {
    SendValue { to: ActorId, value: u128 },
    SendValueTwice { to: ActorId, value: u128 },
    AddUrl { code: String, url: String },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Event {
    Added { code: String, url: String },
    Log(String),
    SentValue { to: ActorId, value: u128 },
    SentValueTwice { to: ActorId, value: u128 },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Query {
    All,
    Code(String),
    Whoami,
    BlockNumber,
    BlockTimestamp,
    ProgramId,
    MessageId,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum Reply {
    All(Contract),
    Url(Option<String>),
    Whoami(ActorId),
    BlockNumber(u32),
    BlockTimestamp(u64),
    ProgramId(ActorId),
    MessageId(MessageId),
}
