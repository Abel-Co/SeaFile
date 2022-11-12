// #![allow(warnings)]
// #![allow(unused_variables)] // 允许未使用的变量
// #![allow(dead_code)] // 允许未使用的代码
// #![allow(unused_must_use)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rbatis;
extern crate regex;
// #[macro_use]
// extern crate validator_derive;
// extern crate serde;
// extern crate serde_json;

pub mod boot;
pub mod module;

#[macro_export]
macro_rules! do_loop {(
    $body:block while $cond:expr
) => ({
    let mut first = true;
    while ::core::mem::replace(&mut first, false) || $cond
        $body
})}