#[allow(unused_imports)]
#[macro_use]
pub extern crate log;
pub mod glloger{
    use log4rs::append::console::ConsoleAppender;
    use log4rs::encode::pattern::PatternEncoder;
    use log4rs::append::file::FileAppender;
    use log4rs::Config;
    use log4rs::config::{Appender, Logger, Root};
    use log::LevelFilter;
    #[allow(unused_imports)]
    use log::info;

    pub enum Logtype {
        Ltcolsole,
        Ltfile
    }

    fn init_log(lt: Logtype, filename:&String) {
        let stdout = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("[Console] {d} - {l} -{t} - {m}{n}")))
            .build();

        let mut dir = String::from(filename);
        if dir=="" {
            dir = gl_tools::filehelper::getcurrentpath()+"\\log.log";
        }

        let file = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("[File] {d} - {l} - {t} - {m}{n}")))
            .build(dir)
            .unwrap();

        let config = Config::builder().appender(Appender::builder().build("stdout", Box::new(stdout)))
            .appender(Appender::builder().build("file", Box::new(file)))
            .logger(Logger::builder()
                .appender("file")
                .additive(false)
                .build("app", LevelFilter::Info));
        let tmpstr:String;
        match lt {
            Logtype::Ltcolsole=>{
                tmpstr = String::from("stdout");
            }
            _=>{
                tmpstr = String::from("file");
            }
        }
        let _ = log4rs::init_config(config.build(Root::builder().appender(tmpstr).build(LevelFilter::Info)).unwrap()).unwrap();
    }
    pub fn init_log_file(filename:&String){
        init_log(Logtype::Ltfile,filename);
    }
    pub fn init_log_colsole(){
        let fname = String::from("");
        init_log(Logtype::Ltcolsole,&fname);
    }
    pub fn init_log_bycfg(cfgfile:&String){
        let _ = log4rs::init_file(cfgfile, Default::default());
    }
}