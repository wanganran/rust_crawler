use std::fs::{OpenOptions,Files};
use std::io::{SeekFrom, BufReader};
use std::io::prelude::*; //FUCK: only compiler would tell you use this

pub fn read_list_from_file(path:String)->Vec<(String, u64)>{
    let mut res=vec!();
    let file=OpenOptions::new().read(true).open(path).unwrap();
    let mut buf_reader=BufReader::new(&file);
    let mut unused_line=String::new();
    loop{
        let mut line=String::new();
        match buf_reader.read_line(&mut line){
            Ok(0)=>break, //FUCK: ended with ,
            Ok(_)=>{
                res.push((line.replace("\r","").replace("\n",""), buf_reader.seek(SeekFrom::Current(0)).unwrap()));

                buf_reader.read_line(&mut unused_line).unwrap();
            },
            Err(err)=>{
                println!("err_read: {:?}", err);
            }
        }
    }
    res
}
pub fn read_def_from_file(word:&(String, u64), reader: &mut BufReader<&File>)->Option<String>{
    let mut def=String::new();
    reader.seek(SeekFrom::Start(word.1)).unwrap();
    reader.read_line(&mut def).unwrap();
    println!("{}:\t{}",word.1,def.len());
    if def=="(error)\n" {None} else {Some(def)}
}
