#![feature(plugin)]
#![plugin(maud_macros)]
extern crate maud;

extern crate hyper;
use hyper::server::{Server, Request, Response};

#[macro_use]
extern crate clap;
use clap::{App, Arg};

use std::time::{SystemTime, UNIX_EPOCH};
static mut STARTUP_TIME: SystemTime = UNIX_EPOCH;

fn message(_: Request, res: Response) {
    unsafe {
        let message = match STARTUP_TIME.elapsed() {
            Ok(elapsed) => {
                let minutes = elapsed.as_secs() / 60;
                if minutes < 5 {
                    format!("网站维护中，请稍等{}分钟后回来！", 5 - minutes)
                } else if minutes > 15 {
                    "网站维护中，请等30分钟后回来！".to_string()
                } else {
                    "网站维护中，请稍等片刻！".to_string()
                }
            }
            Err(e) => {
                format!("Error: {:?}", e)
            }
        };

        let markup = html! {
          html {
            head {
              meta charset="utf-8" /
              title "云路网"
            }
            body {
              p (message)
            }
          }
        };

        res.send(markup.into_string().as_bytes()).unwrap();
    }
}

fn main() {
    let arguments = App::new("right-back web server")
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

    unsafe { STARTUP_TIME = SystemTime::now(); }

    if let Some(port) = arguments.value_of("port") {
        let address = format!("0.0.0.0:{}", port);
        println!("right-back web server is running at http://{}/", address);
        Server::http(address.as_str()).unwrap().handle(message).unwrap();
    }
}
