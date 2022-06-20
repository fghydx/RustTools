#[macro_use]
extern crate log;
extern crate log4rs;

use log::LevelFilter;
use log::Level;
use log::info;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

use std::net::{SocketAddr, UdpSocket,IpAddr};
use std::thread::sleep;
use std::time::Duration;

#[cfg(windows)]
fn init_log() {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[Console] {d} - {l} -{t} - {m}{n}")))
        .build();
    let dir = gl_tools::filehelper::getcurrentpath()+"\\log4rs.log";
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("[File] {d} - {l} - {t} - {m}{n}")))
        .build(dir)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .logger(Logger::builder()
            .appender("file")
            .additive(false)
            .build("app", LevelFilter::Info))
        .build(Root::builder().appender("file").build(LevelFilter::Info))
        .unwrap();

    let _ = log4rs::init_config(config).unwrap();
}

fn main(){
    init_log();
    info!("in main");
    let dir = gl_tools::filehelper::getcurrentpath()+ "\\log4rs.yml";
    info!("当前路径:{}",dir);
    let start = ||{   //服务启动时执行
        // init_log();
        // let dir = gl_tools::gltools::filehelper::getcurrentpath()+ "\\log4rs.yml";
        // if let Err(e) = log4rs::init_file(dir, Default::default()){
        //     eprintln!("{}",e);
        // }
        info!("loging");
        info!(target:"app", "File info");
        log!(Level::Error,"error");
        log!(Level::Info,"Info");
        log!(Level::Warn,"Warn");
        log!(Level::Debug,"Debug");
        log!(Level::Trace,"Trace");
        const LOOPBACK_ADDR: [u8; 4] = [127, 0, 0, 1];
        const RECEIVER_PORT: u16 = 1234;
        const PING_MESSAGE: &str = "ping\n";

        let loopback_ip = IpAddr::from(LOOPBACK_ADDR);
        let sender_addr = SocketAddr::new(loopback_ip, 0);
        let receiver_addr = SocketAddr::new(loopback_ip, RECEIVER_PORT);
        let msg = PING_MESSAGE.as_bytes();
        let socket = UdpSocket::bind(sender_addr).unwrap();
        for _i in 0..9
        {
            info!("发送信息:{:#?}",msg);
            let _ = socket.send_to(msg, receiver_addr);
            sleep(Duration::from_secs(1));
        }

    };
    let stop = ||{   //服务停止时执行
    const LOOPBACK_ADDR: [u8; 4] = [127, 0, 0, 1];
        info!("服务终止!");
        const RECEIVER_PORT: u16 = 1234;
        const PING_MESSAGE: &str = "stop\n";
        let loopback_ip = IpAddr::from(LOOPBACK_ADDR);
        let sender_addr = SocketAddr::new(loopback_ip, 0);
        let receiver_addr = SocketAddr::new(loopback_ip, RECEIVER_PORT);
        let msg = PING_MESSAGE.as_bytes();
        let socket = UdpSocket::bind(sender_addr).unwrap();
        let _ = socket.send_to(msg, receiver_addr);
    };
    let _tt = gl_windows::windowservice::create_service("abc",
                                                     &String::from("bcd"),Some(start),Some(stop));
}


#[cfg(not(windows))]
fn main(){

}
