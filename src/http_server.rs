extern crate hyper;

use self::hyper::server::{Server,Request,Response};
use self::hyper::uri::RequestUri;

fn test(req:Request, resp:Response){
    match req.uri{
        RequestUri::AbsolutePath(str) => resp.send(str.clone().as_bytes()),
        RequestUri::AbsoluteUri(_)    => resp.send(b"uri"),
        _                 => resp.send(b"others")
    }.unwrap();
}
pub fn start_server(){
    Server::http("0.0.0.0:1024").unwrap().handle(test).unwrap();
}
