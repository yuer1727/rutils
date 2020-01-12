

#[macro_use]
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

use chrono::{Local, Utc, TimeZone};


#[derive(Debug, StructOpt)]
#[structopt(name = "rutils", about = "make some command utils for effectiveness.", author = "yuer1727. yuer1727@gmail.com", version="0.0.1")]
enum Opt {

    #[structopt(name = "ts", about = "format timestamp by you sended.")]
    Timestamp {
        #[structopt(short = "t", long = "time", help = "timestamp for format.")]
        time: i64
    },
    #[structopt(name = "fetch")]
    Fetch {
        #[structopt(long = "dry-run")]
        dry_run: bool,
        #[structopt(long = "all")]
        all: bool,
        repository: Option<String>
    },
    #[structopt(name = "commit")]
    Commit {
        #[structopt(short = "m")]
        message: Option<String>,
        #[structopt(short = "a")]
        all: bool
    }
}



fn main() {
    let opt = Opt::from_args();
    //println!("{:?}", opt);

    match opt {
        Opt::Timestamp { time } => {
            println!("timestamp:{}", time);

            println!("{}", chrono::Local.timestamp_millis(time).format("%Y-%m-%d %H:%M:%S.%s"));
        },
        Opt::Commit { message, all } => {
            //...
        }
        _ => (),
    }
}
