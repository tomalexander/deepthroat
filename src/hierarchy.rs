use std::path::PathBuf;

use db;

fn get_base() -> PathBuf {
    PathBuf::from("output")
}

pub fn get_room_path(room: &db::DbRoom) -> PathBuf {
    let mut ret = get_base();
    ret.push("rooms");
    ret.push(room.id.to_string());
    ret
}

