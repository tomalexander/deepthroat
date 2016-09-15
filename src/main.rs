#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate rusqlite;
extern crate chrono;
extern crate serde;
extern crate serde_json;

pub mod db;
pub mod output;

fn main() {
    let data = db::Context::new();
    let page_writer: output::PageWriter = output::PageWriter {};
    let dust_engine: output::DustEngine = output::DustEngine::new();
    page_writer.generate_room_list(&dust_engine, &data);
    for room in data.get_rooms() {
        page_writer.generate_date_list(&dust_engine, room);
        for day in &room.days {
            page_writer.generate_message_list(&dust_engine, room, day);
        }
    }
}

