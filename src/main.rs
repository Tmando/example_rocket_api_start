#[macro_use] extern crate rocket;
use rocket::*;
use rocket_dyn_templates::Template;
use rocket_dyn_templates::context;
use rocket::serde::*;
use rocket::{serde::{Deserialize, json::Json}};
use rocket::request::{Outcome, Request, FromRequest};

struct CustomMiddleware;

#[get("/")]
fn hello()->&'static str{
    return "Hello World";
}

#[get("/route_with_parameter?<first_name>&<last_name>&<city>")]
fn get_request_with_parameters(
    first_name:String,
    last_name:String,
    city:String)->Template{
        return Template::render("about",context! {
            first_name:first_name,
            last_name:last_name,
            city:city
        });
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct ArithmeticInput{
    num_one:f64,
    num_two:f64
}

#[post("/add_num/foo",data="<input_dat>")]
fn add_num(input_dat:Json<ArithmeticInput>)->String{
    let res = input_dat.num_one + input_dat.num_two;
    res.to_string()
}

#[get("/add_num/foo")]
fn test_fun()->&'static str{
    return "Test";
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![hello,get_request_with_parameters,add_num,test_fun])
    .attach(Template::fairing())
}

