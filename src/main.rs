#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate rusqlite;
extern crate chrono;
extern crate serde;
extern crate serde_json;

pub mod db;
pub mod hierarchy;

use std::collections::BTreeMap;

fn main() {
    let conn = db::open_db();
    let rooms = db::get_rooms(&conn);
    for room in rooms {
        let messages = db::get_messages_for_room(&conn, &room);
        let grouped_by_date = group_messages_by_date(messages);
        for (date, messages_that_day) in &grouped_by_date {
            println!("{}: {} messages", date, messages_that_day.len());
        }
        // println!("{}", hierarchy::get_room_path(&room).to_string_lossy());
    }
}

fn group_messages_by_date(messages: Vec<db::DbMessage>) -> BTreeMap<String, Vec<db::DbMessage>> {
    let mut ret: BTreeMap<String, Vec<db::DbMessage>> = BTreeMap::new();
    for msg in messages {
        let key: String = msg.get_date_string();
        ret.entry(key).or_insert(Vec::new()).push(msg);
    }
    ret
}
