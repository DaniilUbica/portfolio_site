use rocket::post;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use rocket_dyn_templates::{Template, context};
use serde::{Deserialize, Serialize};
use crate::json::{read_single_json, rewrite_single_json};
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

impl Content {
    pub fn update_if_not_empty(&mut self, new_content: Content) {
        if !new_content.about_me.is_empty() {
            self.about_me = new_content.about_me.clone();
        }
        if !new_content.name.is_empty() {
            self.name = new_content.name.clone();
        }
        if !new_content.description.is_empty() {
            self.description = new_content.description.clone();
        }
    }
}

#[post("/admin/content", format = "json", data = "<new_content>")]
pub fn post_content(cookies: &CookieJar, new_content: Json<Content>) -> Template {
    let mut content: Content = read_single_json(CONTENT_DEFAULT_JSON_PATH);
    content.update_if_not_empty(new_content.0);

    match cookies.get_private("user_id") {
        Some(_) => {
            log!(INFO, "Got a new content Json");
            if let Err(err) = rewrite_single_json(&Json(content), CONTENT_JSON_PATH) {
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