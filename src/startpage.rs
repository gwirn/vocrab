use crate::{calc_spacing, clear_screen, left_pad, top_pad};
use core::panic;
use std::io::{self, Write};

pub struct StartPage<'a> {
    pub selected_unit: &'a str,
    pub all_units: Vec<String>,
    pub term_width: f32,
    pub term_height: f32,
}
impl StartPage<'_> {
    pub fn show_and_select(&self) -> usize {
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
        for (ci, i) in self.all_units.iter().enumerate() {
            let selection_string = format!("{} for unit {}", ci, i);
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
        let unit_idx: usize = match inp.trim().parse() {
            Ok(x) => x,
            Err(_) => {
                panic!("Unable to parse input to index");
            }
        };
        if unit_idx > self.all_units.len() - 1 {
            panic!("Given index is out of range for units")
        }
        unit_idx
    }
}
