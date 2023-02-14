use std::{collections::{HashMap, HashSet}};

pub static NAME_DEV_1: &str = "Socket 1";
pub static NAME_DEV_2: &str = "Socket 2";
pub static NAME_DEV_3: &str = "Thermo 1";

pub struct SmartHouse {
    pub name: String,
    rooms: HashMap<String, Room>
}

struct Room {
    devices: HashSet<String>
}

impl SmartHouse {
    pub fn new() -> Self {
        let mut s = SmartHouse {
            name: String::from("My smart_house"),
            rooms: HashMap::new()
        };

        // Adding example rooms
        let mut room_a = Room{devices: HashSet::new()};
        room_a.devices.insert(NAME_DEV_1.to_string());

        let mut room_b = Room{devices: HashSet::new()};
        room_b.devices.insert(NAME_DEV_2.to_string());
        room_b.devices.insert(NAME_DEV_3.to_string());

        s.rooms.insert(String::from("Room A"), room_a);
        s.rooms.insert(String::from("Room B"), room_b);
        s
    }

    pub fn get_rooms(&self) -> [&str; 2] {
        let mut result: [&str; 2] = [""; 2];
        let mut count = 0;
        for (room_name, _) in &self.rooms {
            result[count] = &room_name;
            count += 1;
        };
        result
    }

    pub fn devices(&self, room: &str) -> [&str; 3] {
        let mut result = [""; 3];
        let mut count = 0;
        for dev_name in &self.rooms[room].devices {
            result[count] = &dev_name;
            count += 1;
        };
        result
    }

    pub fn create_report(&self, informer: &impl DeviceInfoProvider) -> String {
        let mut report = String::from("Report: \n");
        for (room, val) in &self.rooms {
            for device in &val.devices {
                let info = informer.get_device_info(room, device);
                report.push_str(&info);
                report.push_str("\n");
            }
        }
        report
    }
}

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, name: &str) -> String;
}