#![allow(dead_code)]
#![allow(unused_variables)]

use clap::Parser;
use exercise::*;
use files::*;
use startpage::*;
use ui::*;
use utils::*;

mod exercise;
mod files;
mod startpage;
mod ui;
mod utils;

/// Vocabulary learning assistant
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// file path where the csv file is stored
    #[arg(short, long, default_value = "")]
    inpath: String,

    /// file path where the cbor file should be stored
    #[arg(short, long, default_value = "")]
    outpath: String,
}

fn main() {
    let args = Args::parse();
    let isinp = !args.inpath.is_empty();
    let isoutp = !args.outpath.is_empty();
    if isinp && isoutp {
        csv_2_cbor(&args.inpath, &args.outpath);
    } else if isinp | isoutp {
        eprintln!("To convert a csv to cbor you need to provide INPATH an OUTPATH")
    } else {
        start()
    }
}
