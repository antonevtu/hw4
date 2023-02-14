// Метка todo - реализовать самостоятельно

// ***** Пример библиотеки "Умный дом" со статическим содержимым

use hw4::user_house;


fn main() {



    // let devices = house.devices(&String::from("Room B"));
    // println!("{} Room B devices: {:?}", house.name, devices);

    // Строим отчёт с использованием `OwningDeviceInfoProvider`.
    let info_provider_1 = user_house::OwningDeviceInfoProvider::new();

    let rooms = info_provider_1.get_rooms();
    println!("{} rooms: {:?}", info_provider_1.house.name, rooms);

    let devices = info_provider_1.devices(&String::from("Room A"));
    println!("{} Room A devices: {:?}", house.name, devices);

    let report1 = info_provider_1.create_report();

    // Строим отчёт с использованием `BorrowingDeviceInfoProvider`.
    // let info_provider_2 = user_house::BorrowingDeviceInfoProvider::new();
    // let report2 = house.create_report(&info_provider_2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    // println!("Report #2: {report2}");
}