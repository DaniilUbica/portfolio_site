use rocket::{FromForm, post, uri};
use rocket::form::Form;
use rocket::http::{CookieJar, Status};
use rocket::response::{status, Flash, Redirect};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::json::rewrite_single_json;
use crate::log;
use crate::log::LogLevel::*;
use crate::login::Login;
use crate::admin;

pub const CONTENT_JSON_PATH: &str = "./static/text_content.json";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Content {
    name: String,
    description: String,
    about_me: String
}

#[post("/admin/content", format = "json", data = "<content>")]
pub fn post_content(cookies: &CookieJar, content: Json<Content>) {
    match cookies.get_private("user_id") {
        Some(_) => {
            log!(INFO, "Got a new content Json");
            rewrite_single_json(&content, CONTENT_JSON_PATH);
        },
        None => {
            status::Custom(Status::Unauthorized, "Unauthorized");
        }
    }
}