use db;

#[derive(Serialize)]
struct MessageContext {
    date: String,
    sender: String,
    body: String
}

#[derive(Serialize)]
pub struct MessageListContext {
    room_name: String,
    date: String,
    messages: Vec<MessageContext>,
}

impl MessageContext {
    pub fn new(message: &db::Message) -> MessageContext {
        let date: String = message.get_date_time_string();
        let body: String = match message.get_body() {
            Some(body) => body.to_owned(),
            None => "".to_owned(),
        };
        MessageContext {
            date: date,
            sender: message.get_sender().to_owned(),
            body: body,
        }
    }
}

impl MessageListContext {
    pub fn new(room: &db::Room, room_day: &db::RoomDay) -> MessageListContext {
        let room_name = room.get_name().to_owned();
        let date = room_day.current_date.clone();
        let messages = room_day.messages.iter().map(MessageContext::new).collect();
        MessageListContext {
            room_name: room_name,
            date: date,
            messages: messages,
        }
    }
}
