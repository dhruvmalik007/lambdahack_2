#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Serialize};
use rocket_cors::{AllowedOrigins, CorsOptions, AllowedHeaders};

#[derive(Serialize)]
struct Message {
    content: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/json")]
fn json() -> Json<Message> {
    Json(Message {
        content: "Hello, JSON!".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:5173"]);

    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: ["GET", "POST"].iter().cloned().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error creating CORS");

    rocket::build()
        .mount("/", routes![index, json])
        .attach(cors)
}
