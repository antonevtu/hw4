use std::collections::{HashMap, HashSet};

pub static NAME_DEV_1: &str = "Socket 1";
pub static NAME_DEV_2: &str = "Socket 2";
pub static NAME_DEV_3: &str = "Thermo 1";

pub struct SmartHouse {
    pub name: String,
    rooms: HashMap<String, Room>,
}

struct Room {
    devices: HashSet<String>,
}

impl SmartHouse {
    /// Creates SmartHouse object with static content
    /// ```
    /// use hw4::smart_house_dir::smart_house;
    /// let house = smart_house::SmartHouse::new();
    /// ```
    pub fn new() -> Self {
        let mut s = SmartHouse {
            name: String::from("My smart_house_dir"),
            rooms: HashMap::new(),
        };

        // Adding example rooms
        let mut room_a = Room {
            devices: HashSet::new(),
        };
        room_a.devices.insert(NAME_DEV_1.to_string());

        let mut room_b = Room {
            devices: HashSet::new(),
        };
        room_b.devices.insert(NAME_DEV_2.to_string());
        room_b.devices.insert(NAME_DEV_3.to_string());

        s.rooms.insert(String::from("Room A"), room_a);
        s.rooms.insert(String::from("Room B"), room_b);
        s
    }

    /// Returns rooms in the house
    /// ```
    /// use hw4::smart_house_dir::smart_house;
    /// let house = smart_house::SmartHouse::new();
    /// let rooms = house.get_rooms();
    /// let comparison = (rooms == ["Room A", "Room B"]) || (rooms == ["Room B", "Room A"]);
    /// assert!(comparison)
    /// ```
    pub fn get_rooms(&self) -> [&str; 2] {
        let mut result: [&str; 2] = [""; 2];
        for (count, room_name) in self.rooms.keys().enumerate() {
            result[count] = room_name;
        }
        result
    }

    /// Returns devices in required room of the house
    /// ```
    /// use hw4::smart_house_dir::smart_house;
    /// let house = smart_house::SmartHouse::new();
    /// let devices = house.devices("Room B");
    /// let comparison = (devices == ["Thermo 1", "Socket 2", ""]) || (devices == ["Socket 2", "Thermo 1", ""]);
    /// assert!(comparison)
    /// ```
    pub fn devices(&self, room: &str) -> [&str; 3] {
        let mut result = [""; 3];
        for (count, dev_name) in self.rooms[room].devices.iter().enumerate() {
            result[count] = dev_name;
        }
        result
    }

    /// Returns report, using user's info provider about devices state
    /// ```
    /// use hw4::smart_house_dir::smart_house;
    /// pub struct SmartSocket {
    ///     pub name: String,
    ///     pub state: String
    /// }
    /// pub struct OwningDeviceInfoProvider {
    ///     socket: SmartSocket,
    /// }
    /// impl smart_house::DeviceInfoProvider for OwningDeviceInfoProvider {
    ///     fn get_device_info(&self, room: &str, name: &str) -> String{
    ///         let info: String;
    ///         if self.socket.name == name {
    ///             info = format!("room: {}, device: {}, state: {}", room, self.socket.name, self.socket.state);
    ///
    ///         } else {
    ///             info = format!("room: {}, device: {}, not found", room, self.socket.name);
    ///         }
    ///         info
    ///     }
    /// }
    /// let house = smart_house::SmartHouse::new();
    /// let socket1 = SmartSocket {name: smart_house::NAME_DEV_1.to_string(), state: String::from("working")};
    /// let info_provider = OwningDeviceInfoProvider{socket: socket1};
    /// let report = house.create_report(&info_provider);
    /// let keywords = ["Room A", "Room B", "Socket 1", "state", "not found"];
    /// let mut result = true;
    /// for word in keywords {
    /// if !report.contains(word) {
    /// result = false;
    /// }
    /// }
    /// assert!(result);
    pub fn create_report(&self, informer: &impl DeviceInfoProvider) -> String {
        let mut report = String::from("Report: \n");
        for (room, val) in &self.rooms {
            for device in &val.devices {
                let info = informer.get_device_info(room, device);
                report.push_str(&info);
                report.push('\n');
            }
        }
        report
    }
}

impl Default for SmartHouse {
    fn default() -> Self {
        Self::new()
    }
}

pub trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, name: &str) -> String;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rooms() {
        let house = SmartHouse::new();
        let rooms = house.get_rooms();
        let comparison = (rooms == ["Room A", "Room B"]) || (rooms == ["Room B", "Room A"]);
        assert!(comparison)
    }

    #[test]
    fn test_devices() {
        let house = SmartHouse::new();
        let devices = house.devices("Room B");
        let comparison =
            (devices == ["Thermo 1", "Socket 2", ""]) || (devices == ["Socket 2", "Thermo 1", ""]);
        assert!(comparison)
    }

    pub struct SmartSocket {
        pub name: String,
        pub state: String,
    }
    pub struct OwningDeviceInfoProvider {
        socket: SmartSocket,
    }
    impl DeviceInfoProvider for OwningDeviceInfoProvider {
        fn get_device_info(&self, room: &str, name: &str) -> String {
            let info: String = if self.socket.name == name {
                format!(
                    "room: {}, device: {}, state: {}",
                    room, self.socket.name, self.socket.state
                )
            } else {
                format!("room: {}, device: {}, not found", room, self.socket.name)
            };
            info
        }
    }

    #[test]
    fn test_report() {
        let house = SmartHouse::new();
        let socket1 = SmartSocket {
            name: NAME_DEV_1.to_string(),
            state: String::from("working"),
        };
        let info_provider = OwningDeviceInfoProvider { socket: socket1 };
        let report = house.create_report(&info_provider);
        let keywords = ["Room A", "Room B", "Socket 1", "state", "not found"];
        let mut result = true;
        for word in keywords {
            if !report.contains(word) {
                result = false;
            }
        }
        assert!(result);
    }
}
