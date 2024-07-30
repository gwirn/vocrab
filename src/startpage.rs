use crate::{
    access_cbor, calc_spacing, clear_screen, exercise_unit, get_vocabs, left_pad, top_pad,
};
use std::path::Path;
use std::{
    collections::HashMap,
    io::{self, Write},
    path::PathBuf,
};
use std::{env, process};
use terminal_size::{terminal_size, Height, Width};

pub struct StartPage<'a> {
    pub selected_unit: String,
    pub all_units: &'a [String],
    pub mean_sr: &'a HashMap<String, f64>,
    pub term_width: f32,
    pub term_height: f32,
    pub vocab_path: PathBuf,
    pub ab: bool,
}
impl StartPage<'_> {
    pub fn show_and_select(&mut self) {
        clear_screen();
        print!(
            "{}",
            top_pad(
                "",
                calc_spacing(self.all_units.len(), self.term_height),
                self.term_height
            )
        );
        println!("Enter number on the left side to select a unit");
        let srs = self.mean_sr;
        for (ci, i) in self.all_units.iter().enumerate() {
            let selection_string = format!(
                "{} for unit {}  Success rate: {:.2}",
                ci,
                i.replace(".cbor", ""),
                srs[i]
            );
            println!(
                "{}",
                left_pad(
                    &selection_string,
                    calc_spacing(selection_string.len(), self.term_width),
                    self.term_width
                )
            )
        }
        print!("Your choice: ");
        if io::stdout().flush().is_ok() {};
        let mut inp = String::new();
        if io::stdin().read_line(&mut inp).is_ok() {}
        let inp_whole = inp.trim();
        self.ab = !inp_whole.ends_with('r');
        let inp_no_r = inp_whole.replace('r', "");
        if inp_whole == ":qa" {
            clear_screen();
            process::exit(0)
        }
        match inp_no_r.parse::<usize>() {
            Ok(x) => {
                if x > self.all_units.len() - 1 {
                    self.show_and_select();
                } else {
                    self.selected_unit = self.all_units[x].clone()
                }
            }
            Err(_) => self.show_and_select(),
        };
    }
}

pub fn mean_sr(vocab_path: PathBuf, all_units: &[String]) -> HashMap<String, f64> {
    let srs: Vec<f64> = all_units
        .iter()
        .map(|x| {
            let word_map = access_cbor(vocab_path.join(x));
            word_map.values().map(|y| y.success_rate).sum::<f64>() / word_map.len() as f64
        })
        .collect();
    let sr_map: HashMap<_, _> = srs
        .clone()
        .iter()
        .zip(all_units)
        .map(|(x, y)| (y.clone(), *x))
        .collect();
    sr_map
}

pub fn start() {
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
        ab: true,
    };
    sp.show_and_select();
    exercise_unit(
        w,
        h,
        vocab_path.join(&sp.selected_unit),
        &sp.selected_unit,
        sp.ab,
    );
}
