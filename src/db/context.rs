use rusqlite::{Connection, SqliteConnection};
use std::path::Path;
use chrono::{self, TimeZone};
use std::collections::BTreeMap;

pub struct Context {
    rooms: Vec<Room>,
}

struct DbRoom {
    id: i32,
    is_archived: bool,
    name: String,
    privacy: String,
    version: String,
}

struct DbMessage {
    room_id: i32,
    id: String,
    color: Option<String>,
    date: i64,
    sender: String,
    message: Option<String>,
    message_format: Option<String>,
}

pub struct Message {
    db_message: DbMessage,
}

pub struct Room {
    db_room: DbRoom,
    days: Vec<RoomDay>,
}

pub struct RoomDay {
    current_date: String,
    messages: Vec<Message>,
    // previous_date: Option<&RoomDay>,
    // next_date: Option<&RoomDay>,
}

impl Context {
    pub fn new() -> Context {
        let connection = SqliteConnection::open(Path::new("data.db")).unwrap();
        let rooms = generate_room_list(&connection);
        Context {
            rooms: rooms,
        }
    }

    pub fn get_rooms(&self) -> &Vec<Room> {
        &self.rooms
    }
}

fn generate_room_list(connection: &Connection) -> Vec<Room> {
    let mut stmt = connection.prepare("SELECT id, is_archived, name, privacy, version FROM rooms;").unwrap();
    let ids = stmt.query_map(&[], |row| {
        DbRoom {
            id: row.get(0),
            is_archived: row.get(1),
            name: row.get(2),
            privacy: row.get(3),
            version: row.get(4),
        }
    }).unwrap();
    let db_rooms: Vec<DbRoom> = ids.map(|room| room.unwrap()).collect();
    
    Vec::new()
}

fn get_messages_for_room(connection: &Connection, room: &DbRoom) {
    let mut stmt = connection.prepare("SELECT room_id, id, color, date, sender, message, message_format FROM messages WHERE room_id=$1 ORDER BY date;").unwrap();
    let messages = stmt.query_map(&[&room.id], |row| {
        DbMessage {
            room_id: row.get(0),
            id: row.get(1),
            color: row.get(2),
            date: row.get(3),
            sender: row.get(4),
            message: row.get(5),
            message_format: row.get(6),
        }
    }).unwrap();

    let messages: Vec<DbMessage> = messages.map(|message| message.unwrap()).collect();
    let grouped = group_messages_by_date(messages);
}

fn group_messages_by_date(messages: Vec<DbMessage>) -> BTreeMap<String, Vec<DbMessage>> {
    let mut ret: BTreeMap<String, Vec<DbMessage>> = BTreeMap::new();
    for msg in messages {
        let key: String = msg.get_date_string();
        ret.entry(key).or_insert(Vec::new()).push(msg);
    }
    ret
}

impl DbMessage {
    pub fn get_date_string(&self) -> String {
        let tz: chrono::FixedOffset = chrono::FixedOffset::west(5 * 3600); // Approximately eastern time
        let naive_time: chrono::NaiveDateTime = chrono::NaiveDateTime::from_num_seconds_from_unix_epoch(self.date, 0);
        let eastern_time: chrono::DateTime<chrono::FixedOffset> = tz.from_utc_datetime(&naive_time);
        eastern_time.format("%Y%m%d").to_string()
    }
}
