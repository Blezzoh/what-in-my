#![feature(plugin, custom_derive)]
#![feature(decl_macro)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate tera;

use rocket_contrib::Template;
use tera::Context;

#[get("/")]
fn index() -> Template {
    let mut context = Context::new();

    context.add("my_message", &String::from("Heya from template context!"));
    Template::render("layout", &context)

}

fn main() {
    rocket::ignite()
		.mount("/",routes![index])
		.attach(Template::fairing())
		.launch();
}
