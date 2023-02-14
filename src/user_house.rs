use crate::smart_house::smart_house;

// Пользовательские устройства:
struct SmartSocket {
    name: String,
    state: String
}
struct SmartThermometer {
    name: String,
    temperature: f32
}

// Пользовательские поставщики информации об устройствах.
// Могут как хранить устройства, так и заимствывать.
pub struct OwningDeviceInfoProvider {
    socket: SmartSocket,
    pub house: smart_house::SmartHouse
}

impl OwningDeviceInfoProvider {
    pub fn new() -> OwningDeviceInfoProvider {

        // Инициализация устройств
        let socket1 = SmartSocket {name: smart_house::NAME_DEV_1.to_string(), state: String::from("working")};

        OwningDeviceInfoProvider {
            socket: socket1,
            house: smart_house::SmartHouse::new()
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

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
    pub house: smart_house::SmartHouse
}

impl BorrowingDeviceInfoProvider<'static, 'static> {
    pub fn new() -> BorrowingDeviceInfoProvider<'static, 'static> {

        // Инициализация устройств
        let socket2 = SmartSocket {name: smart_house::NAME_DEV_2.to_string(), state: String::from("broken")};
        let thermo = SmartThermometer {name: smart_house::NAME_DEV_3.to_string(), temperature: 25.4};

        BorrowingDeviceInfoProvider {
            socket: &socket2,
            thermo: &thermo,
            house: smart_house::SmartHouse::new()
        }
    }

    pub fn create_report(&self) -> String {
        self.house.create_report(self)
    }

    pub fn get_rooms(&self) -> [&str; 2] {
        self.house.get_rooms()
    }

    pub fn get_devices(&self) -> [&str; 3] {
        self.house.devices(&String::from("Room B"))
    }
}

impl smart_house::DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_device_info(&self, room: &str, name: &str) -> String{
        let info: String;
        if self.socket.name == name {
            info = format!("room: {}, device: {}, state: {}", room, self.socket.name, self.socket.state);

        } else {
            info = format!("room: {}, device: {}, not found", room, self.socket.name);
        }
        info
    }
}

impl <'a, 'b> smart_house::DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_device_info(&self, room: &str, name: &str) -> String {
        let info: String;
        if self.socket.name == name {
            info = format!("room: {}, device: {}, state: {}", room, self.socket.name, self.socket.state);
        } else if self.thermo.name == name {
            info = format!("room: {}, device: {}, state: {}°C", room, self.thermo.name, self.thermo.temperature);
        } else {
            info = format!("room: {}, device: {}, not found", room, self.socket.name);
        }
        info
    }
}






