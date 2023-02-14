// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым


use hw4::smart_house::smart_house;

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
struct OwningDeviceInfoProvider {
    socket: SmartSocket,
}
struct BorrowingDeviceInfoProvider<'a, 'b> {
    socket: &'a SmartSocket,
    thermo: &'b SmartThermometer,
}

// todo: реализация трейта `DeviceInfoProvider` для поставщиков информации

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

fn main() {
    // Инициализация устройств
    let socket1 = SmartSocket {name: smart_house::NAME_DEV_1.to_string(), state: String::from("working")};
    let socket2 = SmartSocket {name: smart_house::NAME_DEV_2.to_string(), state: String::from("broken")};
    let thermo = SmartThermometer {name: smart_house::NAME_DEV_3.to_string(), temperature: 25.4};

    // Инициализация дома
    let house = smart_house::SmartHouse::new();

    let rooms = house.get_rooms();
    println!("{} rooms: {:?}", house.name, rooms);

    let devices = house.devices(&String::from("Room A"));
    println!("{} Room A devices: {:?}", house.name, devices);

    let devices = house.devices(&String::from("Room B"));
    println!("{} Room B devices: {:?}", house.name, devices);

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = OwningDeviceInfoProvider {
        socket: socket1,
    };

    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report1 = house.create_report(&info_provider_1);

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    let info_provider_2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}