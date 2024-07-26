#![allow(dead_code)]
#![allow(unused_variables)]
use std::env;
use std::path::Path;

use exercise::*;
use files::*;
use startpage::*;
use terminal_size::{terminal_size, Height, Width};
use ui::*;
use utils::*;

mod exercise;
mod files;
mod startpage;
mod ui;
mod utils;

fn start() {
    let vocrab_base = match env::var("VOCRAB") {
        Ok(p) => p,
        Err(_) => {
            eprintln!(
                "Unable to access VOCRAB environment variable - assuming current directory as base"
            );
            match env::current_dir() {
                Ok(p) => p.to_str().expect("Unable to get current dir").to_string(),
                Err(_) => ".".to_string(),
            }
        }
    };
    let vocrab_base_path = Path::new(&vocrab_base);

    let size = terminal_size();
    let (w, h): (usize, usize) = match size {
        Some((Width(ww), Height(hh))) => (ww.into(), hh.into()),
        None => (0, 0),
    };
    let mut vocabs = get_vocabs(vocrab_base_path);
    let vocab_path = vocrab_base_path.join("vocabulary");
    let srs = mean_sr(vocab_path.clone(), &vocabs);
    vocabs.sort_by(|a, b| srs[a].total_cmp(&srs[b]));

    let mut sp = StartPage {
        selected_unit: "".to_string(),
        all_units: &vocabs.clone(),
        mean_sr: &srs,
        term_width: w as f32,
        term_height: h as f32,
        vocab_path: vocab_path.clone(),
    };
    let unit = sp.show_and_select();
    exercise_unit(w, h, vocab_path.join(&sp.selected_unit), &sp.selected_unit);
}
fn main() {
    // csv_2_cbor("../vocabulary/test.csv", "../vocabulary/Unit2.cbor");
    start()
}
