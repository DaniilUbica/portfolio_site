#[macro_use] extern crate rocket;

use std::collections::HashMap;
use portfolio::{error::error::Error, error::error::AppError};
use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer, relative};

extern crate portfolio;
use crate::portfolio::api::api::get_repos;

fn err_to_str(err: &Error) -> &'static str {
    match err.error {
        AppError::ApiGetReposJsonError => "Error getting repositories json from GitHub",
        AppError::ApiGetResponseError => "Error getting response from GitHub",
        AppError::ApiRequestSendError => "Error sending request to GitHub",
        AppError::NotFoundError => "Page not Found",
        AppError::ApiKeyNotFoundError => "No github key found",
        AppError::ApiUsernameNotFoundError => "No github username found"
    }
}

#[get("/")]
async fn index() -> Template {
    let repos = match get_repos().await {
        Ok(repos) => repos,
        Err(err) => {
            return Template::render("error", context! { error_message: err_to_str(&err), 
                    error_concrete_message: err.error_text.clone() });
        }
    };

    let mut context = HashMap::new();
    context.insert("repos", &repos);

    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
