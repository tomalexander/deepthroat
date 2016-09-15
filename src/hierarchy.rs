use std::path::PathBuf;

use db;
use output_old;

fn get_base() -> PathBuf {
    PathBuf::from("output")
}

pub fn get_room_list_path() -> PathBuf {
    let mut ret = get_base();
    ret.push("rooms");
    ret
}

pub fn get_room_list_index() -> PathBuf {
    let mut ret = get_room_list_path();
    ret.push("index.html");
    ret
}

pub fn get_room_path(room: &db::DbRoom) -> PathBuf {
    let mut ret = get_base();
    ret.push("rooms");
    ret.push(room.id.to_string());
    ret
}

pub fn get_room_index(room: &db::DbRoom) -> PathBuf {
    let mut ret = get_room_path(room);
    ret.push("index.html");
    ret
}

pub fn get_room_link(room: &db::DbRoom) -> PathBuf {
    let mut ret = PathBuf::from("/rooms");
    ret.push(room.id.to_string());
    ret
}

pub fn get_room_day_link(room: &db::DbRoom, room_day: &output_old::RoomDay) -> PathBuf {
    let mut ret = get_room_link(room);
    ret.push(&room_day.current_date);
    ret
}
