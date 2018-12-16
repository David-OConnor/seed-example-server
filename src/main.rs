#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use rocket::http::Method;
extern crate rocket_cors;
use rocket_cors::{AllowedOrigins, AllowedHeaders};

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
    let (allowed_origins, failed_origins) = AllowedOrigins::some(
        &["https://seed-example.herokuapp.com", "https://seed-rs.org"]
        );
    assert!(failed_origins.is_empty());

    let options = rocket_cors::Cors {
        allowed_origins: allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };


    rocket::ignite()
        .mount("/", routes![index, data_api])
        .attach(options)
        .launch();
}