use db;
use std::collections::BTreeMap;
use std::fs;
use hierarchy;
use room_list;
use dust_executor;
use std::io::Write;

static ROOM_LIST_TEMPLATE: &'static str = include_str!("../offline/dust/room_list.dust");

#[derive(Debug)]
pub struct RoomDay {
    previous_date: Option<String>,
    current_date: String,
    next_date: Option<String>,
    messages: Vec<db::DbMessage>,
}

pub fn process_messages_for_room(room: &db::DbRoom, messages: Vec<db::DbMessage>) -> Vec<RoomDay> {
    let grouped_by_date = group_messages_by_date(messages);
    let mut room_days: Vec<RoomDay> = Vec::new();
    let mut previous_date: Option<String> = None;
    for (date, messages_that_day) in grouped_by_date {
        match room_days.last_mut() {
            Some(ref mut prev_room_day) => {
                prev_room_day.next_date = Some(date.clone());
            },
            None => ()
        };
        room_days.push(RoomDay {
            previous_date: previous_date.clone(),
            current_date: date.clone(),
            next_date: None,
            messages: messages_that_day,
        });
        previous_date = Some(date);
    }
    room_days
}

fn group_messages_by_date(messages: Vec<db::DbMessage>) -> BTreeMap<String, Vec<db::DbMessage>> {
    let mut ret: BTreeMap<String, Vec<db::DbMessage>> = BTreeMap::new();
    for msg in messages {
        let key: String = msg.get_date_string();
        ret.entry(key).or_insert(Vec::new()).push(msg);
    }
    ret
}

pub fn generate_room_list_page(rooms: &Vec<db::DbRoom>) {
    let output_path = hierarchy::get_room_list_path();
    fs::create_dir_all(&output_path);
    let output_index = hierarchy::get_room_list_index();
    let context: room_list::RoomListContext = room_list::RoomListContext::new(rooms);
    let rendered = dust_executor::render_template(&context, ROOM_LIST_TEMPLATE);
    let mut f = fs::File::create(&output_index).unwrap();
    f.write_all(rendered.as_bytes()).unwrap();
}
