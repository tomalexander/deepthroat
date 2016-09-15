use hierarchy;
use db_old;

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
    pub fn new(db_rooms: &Vec<db_old::DbRoom>) -> RoomListContext {
        let list: Vec<RoomContext> = db_rooms.iter().map(RoomContext::new).collect();
        RoomListContext {
            rooms: list,
        }
    }
}

impl RoomContext {
    pub fn new(room: &db_old::DbRoom) -> RoomContext {
        let link: String = hierarchy::get_room_link(room).to_str().unwrap().to_owned();
        RoomContext {
            name: room.name.clone(),
            link: link,
        }
    }
}
