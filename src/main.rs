extern crate hyper;
use hyper::server::{Server, Request, Response};

#[macro_use]
extern crate clap;
use clap::{App, Arg};

use std::time::{SystemTime, UNIX_EPOCH};
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
    let matches = App::new("right-back web server")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .help("Sets http port number")
            .takes_value(true)
            .use_delimiter(false)
            .default_value("80"))
        .get_matches();

    unsafe { startup_time = SystemTime::now(); }

    if let Some(port) = matches.value_of("port") {
        let address = format!("0.0.0.0:{}", port);
        println!("right-back web server is running at http://{}/", address);
        Server::http(address.as_str()).unwrap().handle(message).unwrap();
    }
}
