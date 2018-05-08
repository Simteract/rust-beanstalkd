//! # Easy-to-use beanstalkd client for Rust (IronMQ compatible)

#[macro_use]
extern crate failure;

pub use beanstalkd::Beanstalkd;

mod beanstalkd;
mod commands;
mod error;
mod parse;
mod request;
mod response;
