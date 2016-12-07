extern crate hyper;

use std::time::{SystemTime, UNIX_EPOCH};
use hyper::server::{Server, Request, Response};

static mut startup_time: SystemTime = UNIX_EPOCH;

fn message(req: Request, res: Response) {
   unsafe { match startup_time.elapsed() {
       Ok(elapsed) => {
           let minutes = elapsed.as_secs() / 60;
           if minutes <= 5 {
               res.send(format!("网站维护中，请稍等{}分钟后回来！", 5 - minutes).as_bytes());
           } else if minutes > 15 {
               res.send("网站维护中，请等30分钟后回来！".as_bytes());
           } else {
               res.send("网站维护中，请稍等片刻！".as_bytes());
           }
       }
       Err(e) => {
           res.send(format!("Error: {:?}", e).as_bytes());
       }
   }}
}

fn main() {
    println!("Start right-back server...");
    unsafe { startup_time = SystemTime::now(); }
    Server::http("0.0.0.0:3000").unwrap().handle(message).unwrap();
}
