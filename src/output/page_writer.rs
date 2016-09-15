use db;
use std::fs;
use std::io::Write;

pub struct PageWriter {

}

impl PageWriter {
    pub fn generate_room_list(&self, engine: &super::DustEngine, context: &db::Context) {
        let paths: db::OutputRoomList = context.path_room_list();
        fs::create_dir_all(&paths.get_path());
        let output_index = paths.get_index();
        println!("Generating {}", output_index.to_str().unwrap());
        let dust_context = super::RoomListContext::new(context.get_rooms());
        let rendered = engine.render_template(&dust_context, "room_list");
        let mut f = fs::File::create(&output_index).unwrap();
        f.write_all(rendered.as_bytes()).unwrap();
    }

    pub fn generate_date_list(&self, engine: &super::DustEngine, room: &db::Room) {
        let paths: db::OutputRoom = room.path_room();
        fs::create_dir_all(&paths.get_path());
        let output_index = paths.get_index();
        println!("Generating {}", output_index.to_str().unwrap());
        let dust_context = super::DateListContext::new(room);
        let rendered = engine.render_template(&dust_context, "date_list");
        let mut f = fs::File::create(&output_index).unwrap();
        f.write_all(rendered.as_bytes()).unwrap();
    }

    pub fn generate_message_list(&self, engine: &super::DustEngine, room: &db::Room, room_day: &db::RoomDay) {
        let paths: db::OutputDateList = room_day.path_date_list();
        fs::create_dir_all(&paths.get_path());
        let output_index = paths.get_index();
        println!("Generating {}", output_index.to_str().unwrap());
        let dust_context = super::MessageListContext::new(room, room_day);
        let rendered = engine.render_template(&dust_context, "message_list");
        let mut f = fs::File::create(&output_index).unwrap();
        f.write_all(rendered.as_bytes()).unwrap();
    }
}
