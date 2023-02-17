use hw4::*;

#[test]
fn test_owning_provider() {
    let report = owning_house::run_owning_provider();
    let keywords = ["Room A", "Room B", "Socket 1", "state", "not found"];
    let mut result = true;
    for word in keywords {
        if !report.contains(word) {
            result = false;
        }
    }
    assert!(result);
}

#[test]
fn test_borrowing_provider() {
    let report = borrowing_house::run_borrowing_provider();
    let keywords = [
        "Room A",
        "Room B",
        "Socket 2",
        "Thermo 1",
        "state",
        "not found",
        "25.4°C",
    ];
    let mut result = true;
    for word in keywords {
        if !report.contains(word) {
            result = false;
        }
    }
    assert!(result);
}
