extern crate hyper;
extern crate regex;
extern crate encoding;
extern crate thread_scoped;

use self::hyper::client::Client;
use self::hyper::status::StatusCode;
use self::regex::Regex;
use std::io::Read;
use self::encoding::Encoding;

fn get_word(url:String, content_regex: &Regex, client: &Client)->Option<String>{
    match client.get(&url).send(){
        Err(err) => {
            println!("{:?}",err);
            None
        },
        Ok(mut res) => {
            if res.status==StatusCode::Ok{ //FUCK: can't be guard)
                let mut word_raw_arr=vec!();
                match res.read_to_end(&mut word_raw_arr){
                    Err(err) =>{
                        println!("err3: {:?}",err);
                        None
                    }
                    Ok(len)  =>{
                        println!("Processed {}, {} bytes.", url, len);
                        let word_content=encoding::all::GBK.decode(&word_raw_arr,encoding::DecoderTrap::Strict).unwrap(); //FUCK: no inbuilt regex, encoding, http servers.
                        match content_regex.captures(&word_content){
                            Some(res) => res.at(0).map(|str| str.to_string()),
                            None => None
                        }
                    }
                }
            }
            else{
                None
            }
        }
    }
}
pub fn crawl_list(url:String) ->Option<Vec<(String,String)>>{
    let client=Client::new();
    let list_raw=client.get(&url).send();
    match list_raw{
        Err(err) => {
            println!("err1: {:?}", err);
            None
        },
        Ok(mut res)  => if res.status==StatusCode::Ok {
            let mut list_raw_arr=vec!();
            match res.read_to_end(&mut list_raw_arr) {
                Err(err) => {
                    println!("err2: {:?}",err);
                    None
                },
                Ok(_)  => {
                    let list_content=encoding::all::GBK.decode(&list_raw_arr,encoding::DecoderTrap::Strict).unwrap();
                    let regex_list=Regex::new(r#"href="([a-z/. -]+)">([a-z- .&]+)</a>"#).unwrap(); //GOOD:string
                    Some(regex_list.captures_iter(&list_content).map(|entry|{
                            let url_rel=entry.at(1).unwrap();
                            let word=entry.at(2).unwrap().to_string();
                            let url_parent=url.trim_right_matches(|c| c!='/');
                            let full_url=url_parent.to_string()+url_rel;
                            (word, full_url)
                    }).collect::<Vec<_>>())
                }
            }
        }
        else{
            None
        }
    }
}
pub fn crawl_words(arr: &Vec<(String, String)>, begin:usize, limit:usize)->Option<Vec<(String, Option<String>)>>{
    let client=Client::new();
    let regex_word=Regex::new(r#"<table ([^a]|a)+</table>\r"#).unwrap();
    unsafe{ //FUCK: thread_scoped must be unsafe, and deprecated
        Some(arr.into_iter().skip(begin).take(limit).map(|tuple|{
            let word=&tuple.0;
            let full_url=&tuple.1;
            (word.clone(), full_url.clone(), &regex_word, &client) //FUCK: need two-stage map to translate between reference and non-reference
        }).map(|(word,full_url, regex_word, client)|{
            (word, thread_scoped::scoped(move || get_word(full_url, regex_word, client)))
        }).map(|(w,g)|(w,g.join())).collect::<Vec<_>>()) //FUCK: ugly ::
    }
}

pub fn crawl(url:String, begin:usize, limit:usize) -> Option<Vec<(String, Option<String>)>>{
    let client=Client::new();
    let list_raw=client.get(&url).send();
    match list_raw{
        Err(err) => {
            println!("err1: {:?}", err);
            None
        },
        Ok(mut res)  => if res.status==StatusCode::Ok {
            let mut list_raw_arr=vec!();
            match res.read_to_end(&mut list_raw_arr) {
                Err(err) => {
                    println!("err2: {:?}",err);
                    None
                },
                Ok(_)  => {
                    unsafe{
                        let list_content=encoding::all::GBK.decode(&list_raw_arr,encoding::DecoderTrap::Strict).unwrap(); //FUCK: many unwrap
                        let regex_list=Regex::new(r#"href="([a-z/. -]+)">([a-z- .&]+)</a>"#).unwrap();
                        let regex_word=Regex::new(r#"<table ([^a]|a)+</table>\r"#).unwrap();
                        return Some(regex_list.captures_iter(&list_content).skip(begin).take(limit).map(|entry|{
                            let url_rel=entry.at(1).unwrap();
                            let word=entry.at(2).unwrap().to_string();
                            let url_parent=url.trim_right_matches(|c| c!='/');
                            let full_url=url_parent.to_string()+url_rel;
                            (word, full_url, &regex_word, &client)
                        }).map(|(word,full_url, regex_word, client)|{
                            (word, thread_scoped::scoped(move || get_word(full_url, regex_word, client)))
                        }).map(|(w,g)|(w,g.join())).collect::<Vec<_>>())
                    }
                }
            }
        } else {
            None
        }
    }
}                   

