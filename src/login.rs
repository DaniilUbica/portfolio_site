use std::collections::HashMap;
use std::env;
use rocket::{FromForm, get, post, uri};
use rocket::form::Form;
use rocket::http::CookieJar;
use rocket::response::{Flash, Redirect};
use rocket_dyn_templates::Template;

use crate::{admin, log};
use crate::log::LogLevel::*;

#[derive(FromForm)]
pub struct Login {
    pub username: String,
    pub password: String,
}

#[get("/login")]
pub fn login_get() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("login", &context)
}

#[post("/login", data = "<form>")]
pub fn login_post(cookies: &CookieJar, form: Form<Login>) -> Result<Redirect, Flash<Redirect>> {
    let pass = env::var("ADMIN_PASSWORD").unwrap();

    if form.username == "admin" && form.password == pass {
        cookies.add_private(("user_id", "admin"));
        Ok(Redirect::to(uri!(admin::admin_get)))
    }
    else {
        log!(INFO, "Somebody tried to login as admin!");
        Err(Flash::error(Redirect::to(uri!(login_get)), "Invalid username or password"))
    }
}