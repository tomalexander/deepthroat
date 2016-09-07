use rusqlite::{Connection, SqliteConnection};
use std::path::Path;
use time;

#[derive(Debug)]
pub struct DbRoom {
    id: i32,
    is_archived: bool,
    name: String,
    privacy: String,
    version: String,
}

#[derive(Debug)]
pub struct DbMessage {
    
}

pub fn open_db() -> Connection {
    SqliteConnection::open(Path::new("data.db")).unwrap()
}

pub fn get_rooms(conn: &Connection) -> Vec<DbRoom> {
    let mut stmt = conn.prepare("SELECT id, is_archived, name, privacy, version FROM rooms;").unwrap();
    let ids = stmt.query_map(&[], |row| {
        DbRoom {
            id: row.get(0),
            is_archived: row.get(1),
            name: row.get(2),
            privacy: row.get(3),
            version: row.get(4),
        }
    }).unwrap();

    let ret: Vec<DbRoom> = ids.map(|room| room.unwrap()).collect();
    ret
}

pub fn get_messages_for_room(conn: &Connection, room: &DbRoom) {
    let mut stmt = conn.prepare("SELECT room_id, id, color, date, sender, message, message_format FROM messages WHERE room_id=$1 ORDER BY date;").unwrap();
    let mut messages = stmt.query_map(&[&room.id], |row| {
        let res: i64 = row.get(0);
        res
    }).unwrap();
}
