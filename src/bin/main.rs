#[macro_use] extern crate rocket;
extern crate portfolio;

use std::collections::HashMap;
use portfolio::{error::error::Error, error::error::AppError};
use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer, relative};
use rocket::Request;
use rocket::serde::Serialize;

use crate::portfolio::api::api::get_repos;
use crate::portfolio::log;
use crate::portfolio::current_function;
use crate::portfolio::log::LogLevel::*;

use portfolio::content::{post_content, CONTENT_JSON_PATH, Content};
use portfolio::login::{login_get, login_post};
use portfolio::admin::admin_get;
use portfolio::api::api::Repository;
use portfolio::json::read_single_json;

#[derive(Serialize)]
struct TemplateContext {
    repos: Vec<Repository>,
    content: Content,
}


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

#[catch(404)]
async fn catcher_404(req: &Request<'_>) -> Template {
    log!(FATAL, format!("Error 404: {}", req.uri()));
    return Template::render("error", context! { error_message: "Error 404" });
}

#[catch(401)]
async fn catcher_401(req: &Request<'_>) -> Template {
    log!(FATAL, format!("Error 401: {}", req.uri()));
    return Template::render("error", context! { error_message: "Error 401" });
}

#[catch(500)]
async fn catcher_500(req: &Request<'_>) -> Template {
    log!(FATAL, format!("Error 500: {}", req.uri()));
    return Template::render("error", context! { error_message: "Error 500" });
}

#[get("/")]
async fn index() -> Template {
    log!(INFO, "Showing the main page");

    let content: Content = read_single_json(CONTENT_JSON_PATH);
    let repos = match get_repos().await {
        Ok(repos) => {
            repos
        },
        Err(err) => {
            return Template::render("error", context! { error_message: err_to_str(&err),
                error_concrete_message: err.error_text.clone() });
        }
    };

    let context = TemplateContext {
        repos,
        content
    };

    Template::render("index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![catcher_401, catcher_404, catcher_500])
        .mount("/", routes![index, post_content, login_get, login_post, admin_get])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
