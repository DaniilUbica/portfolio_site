#[macro_use] extern crate rocket;
extern crate portfolio;

use rocket_dyn_templates::{Template, context};
use rocket::fs::{FileServer};
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
use portfolio::file_upload::*;
use portfolio::json::read_single_json;
use portfolio::mail::send_file_message;

#[derive(Serialize)]
struct TemplateContext {
    repos: Vec<Repository>,
    content: Content,
}

#[catch(404)]
async fn catcher_404(req: &Request<'_>) -> Template {
    log!(FATAL, format!("Error 404: {}", req.uri()));
    send_file_message("Fatal error in application", "./logs/app.log");
    return Template::render("error", context! { error_message: "Error 404" });
}

#[catch(401)]
async fn catcher_401(req: &Request<'_>) -> Template {
    log!(FATAL, format!("Error 401: {}", req.uri()));
    send_file_message("Fatal error in application", "./logs/app.log");
    return Template::render("error", context! { error_message: "Error 401" });
}

#[catch(500)]
async fn catcher_500(req: &Request<'_>) -> Template {
    log!(FATAL, format!("Error 500: {}", req.uri()));
    send_file_message("Fatal error in application", "./logs/app.log");
    return Template::render("error", context! { error_message: "Error 500" });
}

#[get("/")]
async fn index() -> Template {
    log!(INFO, "Showing the main page");

    let content = read_single_json::<Content>(CONTENT_JSON_PATH);

    let repos = match get_repos().await {
        Ok(repos) => repos,
        Err(err) => {
            return Template::render("error", context! { error_message: err.to_str(),
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
    // need to check if this env variables exists, else don't start application
    let _ = std::env::var("MAIL").expect("Got no mail env variable");
    let _ = std::env::var("MAIL_PASSWORD").expect("Got no mail password env variable");

    rocket::build()
        .register("/", catchers![catcher_401, catcher_404, catcher_500])
        .mount("/", routes![index, post_content, login_get, login_post, admin_get, upload_resume, file_upload_success, file_upload_error])
        .mount("/static", FileServer::from("static"))
        .attach(Template::fairing())
}
