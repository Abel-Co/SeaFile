#![allow(unused_variables)] // 允许未使用的变量
#![allow(dead_code)] // 允许未使用的代码
#![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;
extern crate regex;
// #[macro_use]
// extern crate validator_derive;
// extern crate serde;
// extern crate serde_json;

#[macro_use]
pub mod boot;
pub mod module;

