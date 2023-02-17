use hw4::borrowing_house::run_borrowing_provider;
use hw4::owning_house::run_owning_provider;

fn main() {
    println!("Owning provider:");
    run_owning_provider();

    println!("Borrowing provider:");
    run_borrowing_provider();
}
