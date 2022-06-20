#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use gl_windows::windowservice::create_service;
use gl_log::{glloger::{init_log_file}};
use rocket::{Config, Request,http::Method};
use rocket::config::Environment;
use rocket::http::{ContentType, Accept, MediaType};

fn main(){
    let tmp = String::from("");
    init_log_file(&tmp);
    let start  = || {
        let mut cfg = Config::new(Environment::Staging);
        cfg.set_port(1114);
        gl_log::log::info!("test");
        rocket::custom(cfg).mount("/index",routes![hello]).launch();
    };
    let stop = ||{

    };

    let _r = create_service("RUSTWebSvrTest", &String::from("测试应用"), Some(start), Some(stop));
}
// http://127.0.0.1:1114/index/test/10
#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    Request::example(Method::Post,"http://192.168.1.23:12341",|req|{
        req.add_header(ContentType::JSON);
        req.add_header(Accept::HTML);
        req.set_method(Method::Get);
        assert_eq!(req.format(), Some(&MediaType::HTML));
        req.set_method(Method::Post);
        assert_eq!(req.format(), Some(&MediaType::JSON));
    });
    format!("Hello, {} year old named {}!", age, name)
}
