

#[macro_use]
extern crate structopt;

extern crate base64;

use std::path::PathBuf;
use structopt::StructOpt;

use chrono::{Local, Utc, TimeZone};

use std::string::String;





#[derive(Debug, StructOpt)]
#[structopt(name = "rutils", about = "make some command utils for effectiveness.", author = "yuer1727. yuer1727@gmail.com", version="0.0.1")]
enum Opt {

    #[structopt(name = "ts", about = "format timestamp by you sended.")]
    Timestamp {
        #[structopt(short = "t", long = "time", help = "timestamp for format.")]
        time: i64
    },
    #[structopt(name = "base64-decode", about = "base64 decode by you sended.")]
    Base64Decode {
        #[structopt(short = "t", long = "text", help = "base64 decode input.")]
        text: String,
    },
    #[structopt(name = "base64-encode", about = "base64 encode by you sended.")]
    Base64Encode {
        #[structopt(short = "t", long = "text", help = "base64 encode input.")]
        text: String,
    },
}



fn main() {
    let opt = Opt::from_args();
    //println!("{:?}", opt);

    match opt {
        Opt::Timestamp { time } => {
            println!("timestamp:{}", time);

            println!("{}", chrono::Local.timestamp_millis(time).format("%Y-%m-%d %H:%M:%S.%s"));
        },
        Opt::Base64Decode { text } => {
            println!("text:{}", text);

            let result = base64::decode(&text);
            let pure_text = match result {
                Ok(pure_text) => pure_text,
                Err(error) => {
                    panic!("base64 decode error: {:?}", error)
                },
            };
            println!("decode result:\n{}", String::from_utf8(pure_text).unwrap());
        },
        Opt::Base64Encode { text } => {
            println!("text:{}", text);

            let result = base64::encode(&text);
            println!("encode result:\n{}", result);
        }
        _ => (),
    }
}
