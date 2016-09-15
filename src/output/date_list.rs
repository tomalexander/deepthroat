use hierarchy;
use db_old;
use output_old;

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
    pub fn new(room: &db_old::DbRoom, grouped_by_day: &Vec<output_old::RoomDay>) -> DateListContext {
        let list: Vec<DateContext> = grouped_by_day.iter().map(|room_day| DateContext::new(room, room_day)).collect();
        DateListContext {
            dates: list,
        }
    }
}

impl DateContext {
    pub fn new(room: &db_old::DbRoom, room_day: &output_old::RoomDay) -> DateContext {
        let link: String = hierarchy::get_room_day_link(room, room_day).to_str().unwrap().to_owned();
        DateContext {
            date: room_day.current_date.clone(),
            link: link,
        }
    }
}
