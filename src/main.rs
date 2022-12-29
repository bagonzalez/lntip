use dotenv::dotenv;
use rocket::fs::{relative, FileServer};
use rocket::routes;
use rocket_dyn_templates::Template;
use std::env;

#[macro_use]
extern crate rocket;
mod lightning;
mod routes;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/public", FileServer::from(relative!("static"))) //<-- Seteamos un directorio para contenido estático
        .mount(
            "/",
            routes![
                routes::index,
                routes::create_invoice,
                routes::lookup_invoice,
                routes::list_invoices
            ],
        )
        .attach(Template::fairing()) // <--
}

#[get("/hola/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hola, tienes {} años y te llamas {}!", age, name)
}
