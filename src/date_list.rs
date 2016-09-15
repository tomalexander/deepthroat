use hierarchy;
use db;
use output;

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
    pub fn new(room: &db::DbRoom, grouped_by_day: &Vec<output::RoomDay>) -> DateListContext {
        let list: Vec<DateContext> = grouped_by_day.iter().map(|room_day| DateContext::new(room, room_day)).collect();
        DateListContext {
            dates: list,
        }
    }
}

impl DateContext {
    pub fn new(room: &db::DbRoom, room_day: &output::RoomDay) -> DateContext {
        let link: String = hierarchy::get_room_day_link(room, room_day).to_str().unwrap().to_owned();
        DateContext {
            date: room_day.current_date.clone(),
            link: link,
        }
    }
}
