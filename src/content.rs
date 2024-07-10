use rocket::post;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket_dyn_templates::{Template, context};
use serde::{Deserialize, Serialize};

use crate::json::rewrite_single_json;
use crate::log;
use crate::log::LogLevel::*;

pub const CONTENT_JSON_PATH: &str = "./static/text_content.json";
pub const CONTENT_DEFAULT_JSON_PATH: &str = "./static/text_content_default.json";

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Content {
    name: String,
    description: String,
    about_me: String
}

#[post("/admin/content", format = "json", data = "<content>")]
pub fn post_content(cookies: &CookieJar, content: Json<Content>) -> Template {
    match cookies.get_private("user_id") {
        Some(_) => {
            log!(INFO, "Got a new content Json");
            if let Err(err) = rewrite_single_json(&content, CONTENT_JSON_PATH) {
                return Template::render("error", context! { error_message: err.to_str(),
                    error_concrete_message: err.error_text.clone() });
            }
            return Template::render("admin", context! {});
        },
        None => {
            return Template::render("error", context! { error_message: "Error 401",
                    error_concrete_message: "User unauthorized" });
        }
    }
}