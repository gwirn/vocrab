use crate::{access_cbor, calc_spacing, clear_screen, left_pad, top_pad};
use std::{
    collections::HashMap,
    io::{self, Write},
    path::PathBuf,
};

pub struct StartPage<'a> {
    pub selected_unit: String,
    pub all_units: &'a [String],
    pub mean_sr: &'a HashMap<String, f64>,
    pub term_width: f32,
    pub term_height: f32,
    pub vocab_path: PathBuf,
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
            let selection_string = format!("{} for unit {}  Success rate: {:.2}", ci, i, srs[i]);
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
        match inp.trim().parse::<usize>() {
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
            word_map.iter().map(|(_, y)| y.success_rate).sum::<f64>() / word_map.len() as f64
        })
        .collect();
    let sr_map: HashMap<_, _> = srs
        .clone()
        .iter()
        .zip(all_units)
        .map(|(x, y)| (y.clone(), x.clone()))
        .collect();
    sr_map
}
