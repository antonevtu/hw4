extern crate hw4;

#[test]
fn test_owning_provider() {
    let report = hw4::run_owning_provider;
    let x = true;
    assert_eq!(x, false)
}

