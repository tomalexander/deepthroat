#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate rusqlite;
extern crate time;
extern crate serde;
extern crate serde_json;

pub mod db;

fn main() {
    println!("Hello, world!");
}
