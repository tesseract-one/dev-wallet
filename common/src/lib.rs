#![feature(iterator_try_collect)]

#[macro_use]
extern crate log;

#[cfg(target_os = "android")]
mod android;

mod settings;
mod error;
mod core;
mod service;
mod request;
mod ui;

pub(crate) use crate::core::Core;