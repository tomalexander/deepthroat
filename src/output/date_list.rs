use db;

#[derive(Serialize)]
struct DateContext {
    date: String,
    link: String,
}

#[derive(Serialize)]
pub struct DateListContext {
    dates: Vec<DateContext>,
}

impl DateListContext {
    pub fn new(room: &db::Room) -> DateListContext {
        let list: Vec<DateContext> = room.days.iter().map(|day| {
            DateContext::new(room, day)
        }).collect();
        DateListContext {
            dates: list,
        }
    }
}

impl DateContext {
    pub fn new(room: &db::Room, room_day: &db::RoomDay) -> DateContext {
        let link = room_day.path_date_list().get_link().to_str().unwrap().to_owned();
        DateContext {
            date: room_day.current_date.clone(),
            link: link,
        }
    }
}
