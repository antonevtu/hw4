use crate::smart_house_dir::smart_house;

// Пользовательские устройства:
pub struct SmartSocket {
    pub name: String,
    pub state: String,
}
// pub struct SmartThermometer {
//     pub name: String,
//     pub temperature: f32
// }

pub struct OwningDeviceInfoProvider {
    socket: SmartSocket,
    pub house: smart_house::SmartHouse,
}

impl OwningDeviceInfoProvider {
    pub fn new() -> OwningDeviceInfoProvider {
        let socket1 = SmartSocket {
            name: smart_house::NAME_DEV_1.to_string(),
            state: String::from("working"),
        };

        OwningDeviceInfoProvider {
            socket: socket1,
            house: smart_house::SmartHouse::new(),
        }
    }

    pub fn create_report(&self) -> String {
        self.house.create_report(self)
    }

    pub fn get_rooms(&self) -> [&str; 2] {
        self.house.get_rooms()
    }

    pub fn get_devices(&self) -> [&str; 3] {
        self.house.devices(&String::from("Room A"))
    }
}

impl Default for OwningDeviceInfoProvider {
    fn default() -> Self {
        Self::new()
    }
}

impl smart_house::DeviceInfoProvider for OwningDeviceInfoProvider {
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

pub fn run_owning_provider() -> String {
    let info_provider_1 = OwningDeviceInfoProvider::new();

    let rooms = info_provider_1.get_rooms();
    println!("{} rooms: {:?}", info_provider_1.house.name, rooms);

    let devices = info_provider_1.get_devices();
    println!(
        "{} Room A devices: {:?}",
        info_provider_1.house.name, devices
    );

    let report1 = info_provider_1.create_report();

    println!("Report #1: {report1}");
    report1
}
