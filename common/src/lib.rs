#![feature(iterator_try_collect)]
#![feature(result_option_inspect)]

//#[macro_use]
//extern crate log;

#[cfg(target_os = "android")]
mod android;

#[cfg(target_os = "ios")]
mod ios;

mod settings;
mod error;
mod core;
mod service;
mod request;
mod ui;

pub(crate) use crate::core::Core;