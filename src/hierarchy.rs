use std::path::PathBuf;

use db;

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

