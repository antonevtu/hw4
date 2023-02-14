// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым

use std::{collections::{HashMap, HashSet}};


static NAME_DEV_1: &str = "Socket 1";
static NAME_DEV_2: &str = "Socket 2";
static NAME_DEV_3: &str = "Thermo 1";

struct SmartHouse {
    name: String,
    rooms: HashMap<String, Room>
}

struct Room {
    devices: HashSet<String>
}

impl SmartHouse {
    fn new() -> Self {
        let mut s = SmartHouse {
            name: String::from("My house"),
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

    fn get_rooms(&self) -> [&str; 2] {
        let mut result: [&str; 2] = [""; 2];
        let mut count = 0;
        for (room_name, _) in &self.rooms {
            result[count] = &room_name;
            count += 1;
        };
        result
    }

    fn devices(&self, room: &str) -> [&str; 3] {
        let mut result = [""; 3];
        let mut count = 0;
        for dev_name in &self.rooms[room].devices {
            result[count] = &dev_name;
            count += 1;
        };
        result
    }

    fn create_report(&self, informer: &impl DeviceInfoProvider) -> String {
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

trait DeviceInfoProvider {
    fn get_device_info(&self, room: &str, name: &str) -> String;
}

// ***** Пример использования библиотеки умный дом:

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

impl DeviceInfoProvider for OwningDeviceInfoProvider {
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

impl <'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
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
    let socket1 = SmartSocket {name: NAME_DEV_1.to_string(), state: String::from("working")};
    let socket2 = SmartSocket {name: NAME_DEV_2.to_string(), state: String::from("broken")};
    let thermo = SmartThermometer {name: NAME_DEV_3.to_string(), temperature: 25.4};

    // Инициализация дома
    let house = SmartHouse::new();

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
    // // todo: после добавления обобщённого аргумента в метод, расскоментировать передачу параметра
    let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}