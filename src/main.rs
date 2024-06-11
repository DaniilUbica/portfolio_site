#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::form::Form;
use rocket_dyn_templates::{Template, context};
use std::sync::{Arc, Mutex};
use rocket::fs::{FileServer, relative};

#[derive(Debug, Clone, Deserialize, Serialize, FromForm)]
struct User {
    name: String,
}

#[derive(Debug, Default)]
struct UserStore {
    users: Arc<Mutex<Vec<User>>>,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context!{ name: "" })
}

#[post("/submit_form", data = "<user>")]
fn submit(user: Form<User>, store: &rocket::State<UserStore>) -> Json<User> {
    let mut users = store.users.lock().unwrap();
    let new_user = User {
        name: user.name.clone(),
    };

    users.push(new_user.clone());
    Json(new_user)
}

#[post("/submit_json", format = "json", data = "<user>")]
fn submit_json(user: Json<User>, store: &rocket::State<UserStore>) -> Json<User> {
    let mut users = store.users.lock().unwrap();
    users.push(user.clone().into_inner());
    user
}

#[get("/users")]
fn get_users(store: &rocket::State<UserStore>) -> Json<Vec<User>> {
    let users = store.users.lock().unwrap();
    Json(users.clone())
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(UserStore::default())
        .mount("/", routes![index, submit, get_users, submit_json])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
}
