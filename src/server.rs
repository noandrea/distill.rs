use regex::Regex;
use rocket;
use rocket::http::{Header, Method, Status};
use rocket::response::Response;
use rocket::State;
use rocket_contrib::json::*;
use rocket_cors;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use serde_json;
use std::io::Cursor;
use std::path::PathBuf;

pub fn start() {
    let allowed_origins = AllowedOrigins::all();
    let options = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Options]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::All,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap(); // TODO

    rocket::ignite()
        .register(catchers![error400, error404])
        .mount("/", routes![root_handler])
        .mount("/", routes![redirect_handler])
        .mount("/", routes![swagger])
        .attach(options)
        //.manage(self)
        .launch();
}


#[macro_export]
macro_rules! http_error {
    {$code:expr, $reason:expr} => {
        return Err(Status::new($code, $reason));
    }
}



/*
 * Node's only endpoint which lives outside of /v2/...
 */
#[get("/")]
fn root_handler() -> Json<serde_json::Value> {
    Json(serde_json::from_str("[123,1232]").unwrap())
}

#[get("/<code>", rank = 1)]
fn redirect_handler(code: String) -> Json<serde_json::Value> {
    Json(serde_json::from_str("[123,1232]").unwrap())
}

#[get("/api")]
fn swagger() -> JsonValue {
    let swagger_str = include_str!("../swagger/swagger.json");
    serde_json::from_str(swagger_str).unwrap()
}

#[catch(400)]
fn error400() -> Json<serde_json::Value> {
    Json(
        serde_json::from_str(
            r#"
{
  "reason": "Invalid input"
}"#,
        )
        .unwrap(),
    )
}

#[catch(404)]
fn error404() -> Json<serde_json::Value> {
    Json(
        serde_json::from_str(
            r#"
{
  "reason": "Not found"
}"#,
        )
        .unwrap(),
    )
}



