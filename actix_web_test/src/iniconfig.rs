use ini::Ini;
use gl_tools::filehelper::getcurrentpath;

pub struct Tcfg {
    pub listen_port:u32,
    pub write_log: bool
}

pub fn initapp() -> Result<Tcfg,String>{
    let mut tmp = Tcfg{ listen_port: 0, write_log: false };
    if let Ok(i) = Ini::load_from_file(getcurrentpath() + "\\conf.ini"){
        if let Ok(i1) = i.get_from_or(Some("CFG"),"ListenPort","1254").parse::<u32>(){
            tmp.listen_port = i1;
        }
        Ok(tmp)
    } else {
        Err("文件不存在".to_string())
    }
}