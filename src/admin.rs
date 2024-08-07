use std::collections::HashMap;
use rocket::{get, uri};
use rocket::http::CookieJar;
use rocket::response::Redirect;
use rocket_dyn_templates::Template;

use crate::login;

#[get("/admin", rank = 1)]
pub fn admin_get(cookies: &CookieJar) -> Result<Template, Redirect> {
    if cookies.get_private("user_id").is_some() {
        let context: HashMap<&str, &str> = HashMap::new();
        Ok(Template::render("admin", &context))
    }
    else {
        Err(Redirect::to(uri!(login::login_get)))
    }
}
