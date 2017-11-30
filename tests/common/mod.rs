extern crate dotenv;
extern crate mockito;

use self::mockito::Mock;

pub fn setup() {
    dotenv::dotenv().ok();
}

pub fn get_mock() -> Mock {
    mockito::mock("GET", "/forecast/")
        .with_body_from_file("./tests/data/forecast_4.json")
        .create()
}
