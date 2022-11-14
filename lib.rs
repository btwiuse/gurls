#![feature(map_try_insert)]
#![feature(trait_alias)]
#![no_std]

use gstd::prelude::*;

pub mod traits {
    use super::*;

    pub trait IText = From<&'static str>
        + Clone
        + fmt::Debug
        + fmt::Display
        + Default
        + hash::Hash
        + PartialEq
        + Eq
        + PartialOrd
        + Ord
        + AsRef<str>
        + AsMut<str>;

    pub trait IBalance = num_traits::Zero
        + num_traits::One
        + num_traits::CheckedAdd
        + num_traits::CheckedSub
        + num_traits::SaturatingAdd
        + num_traits::SaturatingSub
        + num_traits::sign::Unsigned
        + fmt::Debug
        + Copy
        + Clone
        + PartialOrd
        + PartialEq
        + Default
        + From<u32>
        + From<u16>;

    pub trait IAccountId = Eq + Copy + Clone + core::hash::Hash + Ord + fmt::Debug + Default;

    pub trait ITokenId = Eq + Copy + Clone + core::hash::Hash + Ord + fmt::Debug + Default;

    pub trait IConfig: Default {
        type Balance: IBalance;
        type AccountId: IAccountId;
        type TokenId: ITokenId;
        type Text: IText;
        fn origin(&self) -> Self::AccountId;
        fn sender(&self) -> Self::AccountId;
    }

    pub trait GURLS<T: IConfig> {
        fn set_url(&mut self, key: T::Text, value: T::Text);
        fn get_url(&self, key: T::Text) -> Option<T::Text>;
    }
}

pub mod contract {
    use super::*;

    #[derive(Default)]
    pub struct Contract<T: traits::IConfig> {
        pub ctx: T,
        pub urls: BTreeMap<T::Text, T::Text>,
    }

    impl<T: traits::IConfig> Contract<T> {
        pub fn new() -> Self {
            Self::default()
        }
    }

    impl<T: traits::IConfig> traits::GURLS<T> for Contract<T> {
        fn set_url(&mut self, key: T::Text, value: T::Text) {
            self.urls.try_insert(key, value).expect("failed to insert: key already exists");
        }

        fn get_url(&self, key: T::Text) -> Option<T::Text> {
            self.urls.get(&key).cloned()
        }
    }
}

mod config {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct MockConfig {
        pub origin: u32,
        pub sender: u32,
    }

    impl traits::IConfig for MockConfig {
        type Balance = u32;
        type AccountId = u32;
        type TokenId = u32;
        type Text = String;
        fn origin(&self) -> Self::AccountId {
            self.origin
        }
        fn sender(&self) -> Self::AccountId {
            self.sender
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct GearConfig;

    impl traits::IConfig for GearConfig {
        type AccountId = gstd::ActorId;
        type Balance = u128;
        type TokenId = u128;
        type Text = String;
        fn origin(&self) -> Self::AccountId {
            gstd::exec::origin()
        }
        fn sender(&self) -> Self::AccountId {
            gstd::msg::source()
        }
    }
}

mod state {
    use super::*;

    pub static mut STATE: Option<contract::Contract<config::GearConfig>> = None;
}

pub mod codec {
    use super::*;
    use parity_scale_codec::{Decode, Encode};
    use scale_info::TypeInfo;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode, TypeInfo)]
    pub struct Init;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode, TypeInfo)]
    pub struct InitOk;

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode, TypeInfo)]
    pub enum Action {
        SetUrl(String, String),
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode, TypeInfo)]
    pub enum Event {
        SetUrl(String, String),
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode, TypeInfo)]
    pub enum Query {
        GetUrl(String),
    }

    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Encode, Decode, TypeInfo)]
    pub enum State {
        Url(Option<String>),
    }
}

use codec::*;
gstd::metadata! {
    title: "GURLS",
    init:
        input: Init,
        output: InitOk,
    handle:
        input: Action,
        output: Event,
    state:
        input: Query,
        output: State,
}

#[no_mangle]
pub unsafe extern "C" fn init() {
    let _: codec::Init = gstd::msg::load().expect("failed to load Init message");
    state::STATE = Some(contract::Contract::<config::GearConfig>::new());
    gstd::msg::reply(codec::InitOk, 0).expect("failed to reply InitOk");
}

#[no_mangle]
pub unsafe extern "C" fn handle() {
    use traits::GURLS;
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
    use traits::GURLS;
    let query: Query = gstd::msg::load().expect("failed to decode input argument");
    let state = state::STATE.as_ref().expect("failed to get contract state");
    let encoded = match query {
        Query::GetUrl(key) => {
            let value = state.get_url(key);
            State::Url(value)
        }
    }.encode();
    gstd::util::to_leak_ptr(encoded)
}
