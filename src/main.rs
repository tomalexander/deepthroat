#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate rusqlite;
extern crate time;
extern crate serde;
extern crate serde_json;

pub mod db;
pub mod hierarchy;

fn main() {
    let conn = db::open_db();
    let rooms = db::get_rooms(&conn);
    for room in rooms {
        let messages = db::get_messages_for_room(&conn, &room);
        println!("{}", hierarchy::get_room_path(&room).to_string_lossy());
    }
}
