#![no_std]

use gmeta::{metawasm, Metadata};
use gstd::prelude::*;
use io::ProgramMetadata;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

#[metawasm]
pub mod metafns {
    pub type State = <ProgramMetadata as Metadata>::State;

    pub fn get_url(state: State, code: String) -> Option<String> {
        state.get_url(code)
    }
}
