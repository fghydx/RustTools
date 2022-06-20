#[macro_use]
extern crate lazy_static;

mod abc;
mod iniconfig;

use actix_web::{App,web, HttpServer};
use abc::test::yyy;
use iniconfig::{initapp};
use gl_log::glloger::init_log_file;

#[actix_rt::main]
async fn main() -> std::io::Result<()>  {
    init_log_file(&"".to_string());
    let cfg = initapp().unwrap_or(iniconfig::Tcfg{ listen_port: 12456, write_log: false });
    let  s ="127.0.0.1:".to_string() + cfg.listen_port.to_string().as_str();
    println!("{}",s);
    let res = HttpServer::new(|| App::new()
        .service(web::resource("/{route}").route(web::post().to(yyy)))
        .service(abc::test::index))
        .bind(s);
    match res {
        Ok(o) =>o.run().await,
        Err(e) => {
            eprintln!("{}",e);
            Ok(())
        }
    }
}