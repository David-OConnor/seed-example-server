#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize)]
struct Data {
    val: i8,
    text: String,
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/data", format = "application/json")]
fn data_api() -> String {
    let data = Data {
        val: 7,
        text: "Test data".into(),
    };

    serde_json::to_string(&data).unwrap()
}

fn main() {
    rocket::ignite().mount("/", routes![index, data_api]).launch();
}