use std::path::{Path, PathBuf};

fn get_base() -> PathBuf {
    PathBuf::from("output")
}

pub struct OutputRoomList {
    
}

impl OutputRoomList {
    pub fn get_path(&self) -> PathBuf {
        let mut ret = get_base();
        ret.push("rooms");
        ret
    }

    pub fn get_index(&self) -> PathBuf {
        let mut ret = self.get_path();
        ret.push("index.html");
        ret
    }
    
    pub fn get_link(&self) -> PathBuf {
        let mut ret = PathBuf::from("/rooms");
        ret
    }
}

pub struct OutputRoom {
    pub room_id: i32,
}

impl OutputRoom {
    pub fn get_path(&self) -> PathBuf {
        let mut ret = get_base();
        ret.push("rooms");
        ret.push(self.room_id.to_string());
        ret
    }

    pub fn get_index(&self) -> PathBuf {
        let mut ret = self.get_path();
        ret.push("index.html");
        ret
    }

    pub fn get_link(&self) -> PathBuf {
        let mut ret = PathBuf::from("/rooms");
        ret.push(self.room_id.to_string());
        ret
    }
}

pub struct  OutputDateList {
    pub room_id: i32,
    pub date: String,
}

impl OutputDateList {
    pub fn get_path(&self) -> PathBuf {
        let mut ret = get_base();
        ret.push("rooms");
        ret.push(self.room_id.to_string());
        ret.push(&self.date);
        ret
    }

    pub fn get_index(&self) -> PathBuf {
        let mut ret = self.get_path();
        ret.push("index.html");
        ret
    }

    pub fn get_link(&self) -> PathBuf {
        let mut ret = PathBuf::from("/rooms");
        ret.push(self.room_id.to_string());
        ret.push(&self.date);
        ret
    }
}
