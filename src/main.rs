extern crate encoding;

use std::fs::{OpenOptions,File};
use std::io::{SeekFrom, BufReader};
use std::io::prelude::*;

mod crawler;
mod http_server;
mod data_reader;

fn append_to_file(arr:Vec<(String, Option<String>)>, path:String){ //FUCK: here arr can't be reference or we cannot do the pattern matching
    let mut file=OpenOptions::new().create(true).append(true).write(true).open(path).unwrap();
    let crlf="\n".to_string();
    for (w,o) in arr{
        match o{
            None   =>{
                file.write_all(w.as_bytes()).unwrap();
                file.write_all(crlf.as_bytes()).unwrap();
                file.write_all(b"(error)").unwrap();
                file.write_all(crlf.as_bytes()).unwrap();
            }
            Some(s)=>{
                let s_updated=s.replace(r#"<img border=0 src="..\images\point.gif" align=right width=9 height=10>"#, "").replace("\r","").replace("\n","");
                file.write_all(w.as_bytes()).unwrap();
                file.write_all(crlf.as_bytes()).unwrap();
                file.write_all(s_updated.as_bytes()).unwrap();
                file.write_all(crlf.as_bytes()).unwrap();
            }
        }
    }
    file.flush().unwrap();
}


fn exec(comm:String, arg:String)->String{
    use std::process::Command;
    let output = Command::new(comm).arg(arg).output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    String::from_utf8(output.stdout).unwrap()
}
fn refine(old_path:String, new_path:String){
    let list=data_reader::read_list_from_file(old_path.clone());
    let mut newvec=vec!();
    let mut file=OpenOptions::new().read(true).open(old_path.clone()).unwrap();
    let mut buf_reader=BufReader::new(&file);
    for tuple in list.into_iter(){
        match data_reader::read_def_from_file(&tuple, &mut buf_reader){
            None=>{
                println!("{} refining...",tuple.0);
                let newdef=exec("/usr/local/bin/sdcv".to_string(),tuple.0.clone()); //FUCK: tuple element need to be cloned
                println!("{} refined: {} bytes.",tuple.0, newdef.len()); //FUCK: here clone is not needed!
                newvec.push((tuple.0.clone(), Some(newdef)));
            },
            Some(def)=>newvec.push((tuple.0.clone(), Some(def))),
        }
        if newvec.len()>=10 {
            append_to_file(newvec.clone(),new_path.clone()); //FUCK: vector need to be cloned, or it is moved and never come back
            newvec.clear();
        }
    }
    if newvec.len()!=0 {
        append_to_file(newvec.clone(),new_path);
    }
}

fn merge_list(arr1:&Vec<(String,String)>,arr2:&Vec<(String,String)>)->Vec<(String,String)>{
    let mut map=std::collections::btree_map::BTreeMap::new(); //FUCK: no other maps implemented!
    for tuple in arr1.iter() {
        let key=tuple.0.clone();
        if !map.contains_key(&key){
            map.insert(key, tuple);
        }
    }
    for tuple in arr2.iter() {
        let key=tuple.0.clone();
        if !map.contains_key(&key){
            map.insert(key, tuple);
        }
    }
    map.values().map(|tuple| (tuple.0.clone(),tuple.1.clone())).collect::<Vec<_>>() //FUCK: reference cannot use pattern matching
}



fn crawl_and_write(){
    let mut begin=0;
    let once=10;

    let list1=crawler::crawl_list("http://dict.yqie.com/GRE_glossary.htm".to_string()).unwrap();
    let list2=crawler::crawl_list("http://dict.yqie.com/TOFEL_glossary.htm".to_string()).unwrap();
    let list=merge_list(&list1,&list2);
    println!("word count: {}, {}, {}",list1.len(),list2.len(),list.len());
    while begin<list.len(){
        let crawler_result=crawler::crawl_words(&list,begin, once).unwrap();
        append_to_file(crawler_result, "voc.txt".to_string());
        begin+=once;
        println!("{} words written.",begin);
    }
}
fn main(){
    http_server::start_server();
    println!("started.");
    //refine("voc.txt".to_string(),"voc2.txt".to_string());
    //crawl_and_write();
    //let crawler_result=crawler::crawl("http://dict.yqie.com/english_4.htm".to_string(),begin, once).unwrap();
    //append_to_file(crawler_result, "voc.txt".to_string());
}
