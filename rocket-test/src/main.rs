#[macro_use]
extern crate rocket;
use derive_adhoc::{define_derive_adhoc, Adhoc};
use mining::proof::sp1;
use rocket::serde::{json::Json, Serialize};
use rocket::Response;
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};

#[derive(serialize)]
struct ResultGame {
    result: bool,
}

#[get("/submit_proofs")]
fn index(guesses: Vec<(u8, u8)>) -> Response<any, Error> {
    sp1::prove_mine_game(guesses)
}

#[get("/get_result")]
fn call_result_game() -> Response<Json<ResultGame>, Error> {
    let result_game = ResultGame { result: 0 };
    Ok(Json(result_game))
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::all();

    let cors = AdHoc::on_ignite("CORS Config", |rocket| {
        let allowed_origins = AllowedOrigins::all();
        let cors = CorsOptions {
            allowed_origins,
            allowed_headers: AllowedHeaders::all(),
            allow_credentials: true,
            ..Default::default()
        }
        .to_cors()
        .expect("error creating CORS");
        Ok(rocket.manage(cors))
    });

    rocket::build()
        .mount("/submit_proofs", routes![index])
        .attach(cors)
}
