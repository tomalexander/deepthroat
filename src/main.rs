#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate rusqlite;
extern crate chrono;
extern crate serde;
extern crate serde_json;

pub mod db;
pub mod hierarchy;
pub mod output;
pub mod dust_executor;
pub mod room_list;
pub mod date_list;

#[derive(Serialize)]
struct TestContext {
    val: String,
    otherval: String,
}

fn main() {
    let conn = db::open_db();
    let rooms = db::get_rooms(&conn);
    output::generate_room_list_page(&rooms);
    for room in rooms {
        let messages = db::get_messages_for_room(&conn, &room);
        let grouped_by_day: Vec<output::RoomDay> = output::process_messages_for_room(&room, messages);
        output::generate_date_list_page(&room, &grouped_by_day);
    }
    let context = TestContext {
        val: "test".to_owned(),
        otherval: "other".to_owned(),
    };
    dust_executor::render_template(&context, "test dust template {val} afterval");
}

