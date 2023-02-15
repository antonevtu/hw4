use crate::smart_house::smart_house;

// Пользовательские устройства:
pub struct SmartSocket {
    pub name: String,
    pub state: String
}
pub struct SmartThermometer {
    pub name: String,
    pub temperature: f32
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
    pub house: smart_house::SmartHouse
}

impl BorrowingDeviceInfoProvider<'_, '_> {
    pub fn new<'a>(socket: &'a SmartSocket, thermo: &'a SmartThermometer) -> BorrowingDeviceInfoProvider<'a, 'a> {


        BorrowingDeviceInfoProvider {
            socket,
            thermo,
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

pub fn run_borrowing_provider() {
    // Инициализация устройств
    let socket2 = SmartSocket {name: smart_house::NAME_DEV_2.to_string(), state: String::from("broken")};
    let thermo = SmartThermometer {name: smart_house::NAME_DEV_3.to_string(), temperature: 25.4};

    let info_provider_2 = BorrowingDeviceInfoProvider::new(&socket2, &thermo);

    let report2 = info_provider_2.create_report();

    // Выводим отчёты на экран:
    println!("Report #2: {report2}");
}