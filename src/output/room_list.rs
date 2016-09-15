use db;

#[derive(Serialize)]
struct RoomContext {
    name: String,
    link: String,
}

#[derive(Serialize)]
pub struct RoomListContext {
    rooms: Vec<RoomContext>,
}

impl RoomListContext {
    pub fn new(db_rooms: &Vec<db::Room>) -> RoomListContext {
        let list: Vec<RoomContext> = db_rooms.iter().map(RoomContext::new).collect();
        RoomListContext {
            rooms: list,
        }
    }
}

impl RoomContext {
    pub fn new(room: &db::Room) -> RoomContext {
        let link: String = room.path_room().get_link().to_str().unwrap().to_owned();
        RoomContext {
            name: room.get_name().to_owned(),
            link: link,
        }
    }
}
