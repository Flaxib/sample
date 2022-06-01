#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

// use rocket_include_static_resources::{EtagIfNoneMatch, StaticContextManager, StaticResponse};

use rocket_dyn_templates::context;
use rocket_dyn_templates::Template;

static_response_handler! {
    "/favicon.ico" => favicon => "favicon",
}

#[get("/")]
fn index() -> Template {
    Template::render(
        "index",
        context! {
            foo: 123,
            title: "Flaxib Scolaire"
        },
    )
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(static_resources_initializer!(
            "favicon" => "images/favicon.ico",
            // "favicon-png" => "examples/front-end/images/favicon-16.png",
            // "html-readme" => ("examples", "front-end", "html", "README.html"),
        ))
        .attach(Template::fairing())
        .mount("/", routes![favicon])
        .mount("/", routes![index])
}
