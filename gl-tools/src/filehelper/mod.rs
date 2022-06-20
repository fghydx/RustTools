use std::{io,fs};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::{Write, Read, Error, ErrorKind};

pub fn getcurrentpath() -> String {
    std::env::current_exe().unwrap().parent().unwrap().to_str().unwrap().to_string()
}

pub fn create_dir_all(dir : &String)->io::Result<()>{
    fs::create_dir_all(dir)
}

pub fn dir_exists(dir:&String)-> bool{
    let b = Path::new(dir);
    if !b.exists(){
        return false
    }
    true
}

pub fn file_exists(file:&String)->bool{
    if let Ok(_) = File::open(file){
        return true
    }
    return false
}

pub fn create_file(file_name:&String) -> io::Result<File> {
    if let Ok(f) = open_file(file_name) {
        return Ok(f)
    }
    let b = Path::new(file_name).parent();
    if let Some(f) = b {
        if !dir_exists(&f.to_str().unwrap().to_string()) {
            if let Err(e) = fs::create_dir_all(f){
                return Err(e)
            }
        }
    }
    fs::File::create(file_name)
}

pub fn open_file(file_name:&String) -> io::Result<File>{
    fs::File::open(file_name)
}

pub fn remove_file(file_name:&String)->io::Result<()>{
    fs::remove_file(file_name)
}

pub fn remove_dir_all(dir:&String)->io::Result<()>{
    fs::remove_dir_all(dir)
}

pub fn file_append_string(file_name:&String, str:&String) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(file_name)?;
    let b = str.as_bytes();
    if let Err(e) = file.write_all(b) {
        return Err(e)
    }
    Ok(())
}

pub fn file_trunc(file_name:&String) -> io::Result<()>{
    let file = OpenOptions::new().write(true).truncate(true).open(file_name)?;
    file.set_len(0)
}

pub fn read_string(file_name:&String) -> io::Result<String> {
    let mut file = OpenOptions::new().read(true).open(file_name)?;
    let mut str = String::new();
    if let Ok(_) = file.read_to_string(&mut str){
        return Ok(str)
    }
    Err(Error::new(ErrorKind::Other,"read error"))
}

pub fn copy_file(file_src:&String,file_dec:&String)->io::Result<()>{
    let mut file_in = std::fs::File::open(file_src)?;
    let mut file_out = std::fs::File::create(file_dec)?;
    let mut buffer = [0u8; 4096];
    loop {
        if let Ok(nbytes) = file_in.read(&mut buffer) {
            let _ =file_out.write(&buffer[..nbytes]);
            if nbytes < buffer.len() { break; }
        }
    }
    Ok(())
}