#![feature(prelude_import)]
#![feature(map_try_insert)]
#![no_std]
#[prelude_import]
use core::prelude::rust_2021::*;
#[macro_use]
extern crate core;
#[macro_use]
extern crate compiler_builtins;
use gstd::prelude::*;
mod contract {
    use crate::*;
    pub struct Contract(BTreeMap<String, String>);
    #[automatically_derived]
    impl ::core::default::Default for Contract {
        #[inline]
        fn default() -> Contract {
            Contract(::core::default::Default::default())
        }
    }
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
        pub enum Action {
            AddUrl { code: String, url: String },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Action {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Action::AddUrl {
                        code: __self_0,
                        url: __self_1,
                    } => ::core::fmt::Formatter::debug_struct_field2_finish(
                        f, "AddUrl", "code", &__self_0, "url", &__self_1,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Action {
            #[inline]
            fn clone(&self) -> Action {
                match self {
                    Action::AddUrl {
                        code: __self_0,
                        url: __self_1,
                    } => Action::AddUrl {
                        code: ::core::clone::Clone::clone(__self_0),
                        url: ::core::clone::Clone::clone(__self_1),
                    },
                }
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::parity_scale_codec::Encode for Action {
                fn encode_to<
                    __CodecOutputEdqy: ::parity_scale_codec::Output + ?::core::marker::Sized,
                >(
                    &self,
                    __codec_dest_edqy: &mut __CodecOutputEdqy,
                ) {
                    match *self {
                        Action::AddUrl { ref code, ref url } => {
                            __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                            ::parity_scale_codec::Encode::encode_to(code, __codec_dest_edqy);
                            ::parity_scale_codec::Encode::encode_to(url, __codec_dest_edqy);
                        }
                        _ => (),
                    }
                }
            }
            #[automatically_derived]
            impl ::parity_scale_codec::EncodeLike for Action {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::parity_scale_codec::Decode for Action {
                fn decode<__CodecInputEdqy: ::parity_scale_codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::parity_scale_codec::Error> {
                    match __codec_input_edqy.read_byte().map_err(|e| {
                        e.chain("Could not decode `Action`, failed to read variant byte")
                    })? {
                        __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Action::AddUrl {
                                code: {
                                    let __codec_res_edqy =
                                        <String as ::parity_scale_codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Action::AddUrl::code`"),
                                            )
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                },
                                url: {
                                    let __codec_res_edqy =
                                        <String as ::parity_scale_codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Action::AddUrl::url`"),
                                            )
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                },
                            })
                        }
                        _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                            "Could not decode `Action`, variant doesn't exist",
                        )),
                    }
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for Action {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("Action", "gurls::codec::action"))
                        .type_params(::alloc::vec::Vec::new())
                        .variant(::scale_info::build::Variants::new().variant("AddUrl", |v| {
                            v.index(0usize as ::core::primitive::u8).fields(
                                ::scale_info::build::Fields::named()
                                    .field(|f| f.ty::<String>().name("code").type_name("String"))
                                    .field(|f| f.ty::<String>().name("url").type_name("String")),
                            )
                        }))
                }
            };
        };
        pub enum Event {
            Added { code: String, url: String },
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Event {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Event::Added {
                        code: __self_0,
                        url: __self_1,
                    } => ::core::fmt::Formatter::debug_struct_field2_finish(
                        f, "Added", "code", &__self_0, "url", &__self_1,
                    ),
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Event {
            #[inline]
            fn clone(&self) -> Event {
                match self {
                    Event::Added {
                        code: __self_0,
                        url: __self_1,
                    } => Event::Added {
                        code: ::core::clone::Clone::clone(__self_0),
                        url: ::core::clone::Clone::clone(__self_1),
                    },
                }
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::parity_scale_codec::Encode for Event {
                fn encode_to<
                    __CodecOutputEdqy: ::parity_scale_codec::Output + ?::core::marker::Sized,
                >(
                    &self,
                    __codec_dest_edqy: &mut __CodecOutputEdqy,
                ) {
                    match *self {
                        Event::Added { ref code, ref url } => {
                            __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                            ::parity_scale_codec::Encode::encode_to(code, __codec_dest_edqy);
                            ::parity_scale_codec::Encode::encode_to(url, __codec_dest_edqy);
                        }
                        _ => (),
                    }
                }
            }
            #[automatically_derived]
            impl ::parity_scale_codec::EncodeLike for Event {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::parity_scale_codec::Decode for Event {
                fn decode<__CodecInputEdqy: ::parity_scale_codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::parity_scale_codec::Error> {
                    match __codec_input_edqy.read_byte().map_err(|e| {
                        e.chain("Could not decode `Event`, failed to read variant byte")
                    })? {
                        __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Event::Added {
                                code: {
                                    let __codec_res_edqy =
                                        <String as ::parity_scale_codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Event::Added::code`"),
                                            )
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                },
                                url: {
                                    let __codec_res_edqy =
                                        <String as ::parity_scale_codec::Decode>::decode(
                                            __codec_input_edqy,
                                        );
                                    match __codec_res_edqy {
                                        ::core::result::Result::Err(e) => {
                                            return ::core::result::Result::Err(
                                                e.chain("Could not decode `Event::Added::url`"),
                                            )
                                        }
                                        ::core::result::Result::Ok(__codec_res_edqy) => {
                                            __codec_res_edqy
                                        }
                                    }
                                },
                            })
                        }
                        _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                            "Could not decode `Event`, variant doesn't exist",
                        )),
                    }
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for Event {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("Event", "gurls::codec::action"))
                        .type_params(::alloc::vec::Vec::new())
                        .variant(::scale_info::build::Variants::new().variant("Added", |v| {
                            v.index(0usize as ::core::primitive::u8).fields(
                                ::scale_info::build::Fields::named()
                                    .field(|f| f.ty::<String>().name("code").type_name("String"))
                                    .field(|f| f.ty::<String>().name("url").type_name("String")),
                            )
                        }))
                }
            };
        };
    }
    pub mod query {
        use super::*;
        pub enum Query {
            Code(String),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Query {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    Query::Code(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "Code", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Query {
            #[inline]
            fn clone(&self) -> Query {
                match self {
                    Query::Code(__self_0) => Query::Code(::core::clone::Clone::clone(__self_0)),
                }
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::parity_scale_codec::Encode for Query {
                fn encode_to<
                    __CodecOutputEdqy: ::parity_scale_codec::Output + ?::core::marker::Sized,
                >(
                    &self,
                    __codec_dest_edqy: &mut __CodecOutputEdqy,
                ) {
                    match *self {
                        Query::Code(ref aa) => {
                            __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                            ::parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                        }
                        _ => (),
                    }
                }
            }
            #[automatically_derived]
            impl ::parity_scale_codec::EncodeLike for Query {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::parity_scale_codec::Decode for Query {
                fn decode<__CodecInputEdqy: ::parity_scale_codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::parity_scale_codec::Error> {
                    match __codec_input_edqy.read_byte().map_err(|e| {
                        e.chain("Could not decode `Query`, failed to read variant byte")
                    })? {
                        __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(Query::Code({
                                let __codec_res_edqy =
                                    <String as ::parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Query::Code.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }))
                        }
                        _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                            "Could not decode `Query`, variant doesn't exist",
                        )),
                    }
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for Query {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("Query", "gurls::codec::query"))
                        .type_params(::alloc::vec::Vec::new())
                        .variant(::scale_info::build::Variants::new().variant("Code", |v| {
                            v.index(0usize as ::core::primitive::u8).fields(
                                ::scale_info::build::Fields::unnamed()
                                    .field(|f| f.ty::<String>().type_name("String")),
                            )
                        }))
                }
            };
        };
        pub enum State {
            MaybeUrl(Option<String>),
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for State {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    State::MaybeUrl(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(f, "MaybeUrl", &__self_0)
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for State {
            #[inline]
            fn clone(&self) -> State {
                match self {
                    State::MaybeUrl(__self_0) => {
                        State::MaybeUrl(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::parity_scale_codec::Encode for State {
                fn encode_to<
                    __CodecOutputEdqy: ::parity_scale_codec::Output + ?::core::marker::Sized,
                >(
                    &self,
                    __codec_dest_edqy: &mut __CodecOutputEdqy,
                ) {
                    match *self {
                        State::MaybeUrl(ref aa) => {
                            __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                            ::parity_scale_codec::Encode::encode_to(aa, __codec_dest_edqy);
                        }
                        _ => (),
                    }
                }
            }
            #[automatically_derived]
            impl ::parity_scale_codec::EncodeLike for State {}
        };
        #[allow(deprecated)]
        const _: () = {
            #[automatically_derived]
            impl ::parity_scale_codec::Decode for State {
                fn decode<__CodecInputEdqy: ::parity_scale_codec::Input>(
                    __codec_input_edqy: &mut __CodecInputEdqy,
                ) -> ::core::result::Result<Self, ::parity_scale_codec::Error> {
                    match __codec_input_edqy.read_byte().map_err(|e| {
                        e.chain("Could not decode `State`, failed to read variant byte")
                    })? {
                        __codec_x_edqy if __codec_x_edqy == 0usize as ::core::primitive::u8 => {
                            ::core::result::Result::Ok(State::MaybeUrl({
                                let __codec_res_edqy =
                                    <Option<String> as ::parity_scale_codec::Decode>::decode(
                                        __codec_input_edqy,
                                    );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `State::MaybeUrl.0`"),
                                        )
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            }))
                        }
                        _ => ::core::result::Result::Err(<_ as ::core::convert::Into<_>>::into(
                            "Could not decode `State`, variant doesn't exist",
                        )),
                    }
                }
            }
        };
        #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
        const _: () = {
            impl ::scale_info::TypeInfo for State {
                type Identity = Self;
                fn type_info() -> ::scale_info::Type {
                    ::scale_info::Type::builder()
                        .path(::scale_info::Path::new("State", "gurls::codec::query"))
                        .type_params(::alloc::vec::Vec::new())
                        .variant(
                            ::scale_info::build::Variants::new().variant("MaybeUrl", |v| {
                                v.index(0usize as ::core::primitive::u8).fields(
                                    ::scale_info::build::Fields::unnamed().field(|f| {
                                        f.ty::<Option<String>>().type_name("Option<String>")
                                    }),
                                )
                            }),
                        )
                }
            };
        };
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
#[no_mangle]
unsafe extern "C" fn meta_title() -> *mut [i32; 2] {
    let buffer = "github.com/btwiuse/gurls".to_string();
    let result = ::gstd::util::to_wasm_ptr(buffer.as_bytes());
    core::mem::forget(buffer);
    result
}
#[no_mangle]
unsafe extern "C" fn meta_init_input() -> *mut [i32; 2] {
    let buffer = "".to_string();
    let result = ::gstd::util::to_wasm_ptr(buffer.as_bytes());
    core::mem::forget(buffer);
    result
}
#[no_mangle]
unsafe extern "C" fn meta_init_output() -> *mut [i32; 2] {
    let buffer = "".to_string();
    let result = ::gstd::util::to_wasm_ptr(buffer.as_bytes());
    core::mem::forget(buffer);
    result
}
#[no_mangle]
unsafe extern "C" fn meta_async_init_input() -> *mut [i32; 2] {
    let buffer = "".to_string();
    let result = ::gstd::util::to_wasm_ptr(buffer.as_bytes());
    core::mem::forget(buffer);
    result
}
#[no_mangle]
unsafe extern "C" fn meta_async_init_output() -> *mut [i32; 2] {
    let buffer = "".to_string();
    let result = ::gstd::util::to_wasm_ptr(buffer.as_bytes());
    core::mem::forget(buffer);
    result
}
#[no_mangle]
unsafe extern "C" fn meta_handle_input() -> *mut [i32; 2] {
    let buffer = "Action".to_string();
    let result = ::gstd::util::to_wasm_ptr(buffer.as_bytes());
    core::mem::forget(buffer);
    result
}
#[no_mangle]
unsafe extern "C" fn meta_handle_output() -> *mut [i32; 2] {
    let buffer = "Event".to_string();
    let result = ::gstd::util::to_wasm_ptr(buffer.as_bytes());
    core::mem::forget(buffer);
    result
}
#[no_mangle]
unsafe extern "C" fn meta_async_handle_input() -> *mut [i32; 2] {
    let buffer = "".to_string();
    let result = ::gstd::util::to_wasm_ptr(buffer.as_bytes());
    core::mem::forget(buffer);
    result
}
#[no_mangle]
unsafe extern "C" fn meta_async_handle_output() -> *mut [i32; 2] {
    let buffer = "".to_string();
    let result = ::gstd::util::to_wasm_ptr(buffer.as_bytes());
    core::mem::forget(buffer);
    result
}
#[no_mangle]
unsafe extern "C" fn meta_state_input() -> *mut [i32; 2] {
    let buffer = "Query".to_string();
    let result = ::gstd::util::to_wasm_ptr(buffer.as_bytes());
    core::mem::forget(buffer);
    result
}
#[no_mangle]
unsafe extern "C" fn meta_state_output() -> *mut [i32; 2] {
    let buffer = "State".to_string();
    let result = ::gstd::util::to_wasm_ptr(buffer.as_bytes());
    core::mem::forget(buffer);
    result
}
#[no_mangle]
unsafe extern "C" fn meta_registry() -> *mut [i32; 2] {
    let buffer = ::gstd::util::to_hex_registry(<[_]>::into_vec(
        #[rustc_box]
        ::alloc::boxed::Box::new([
            ::gstd::util::MetaType::new::<Action>(),
            ::gstd::util::MetaType::new::<Event>(),
            ::gstd::util::MetaType::new::<Query>(),
            ::gstd::util::MetaType::new::<State>(),
        ]),
    ))
    .to_string();
    let result = ::gstd::util::to_wasm_ptr(buffer.as_bytes());
    core::mem::forget(buffer);
    result
}
