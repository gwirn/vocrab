use crate::Word;

use nu_ansi_term::Color::Fixed;
use std::io::{self, Write};

const GREEN: u8 = 65;
const RED: u8 = 9;
const SINGLE_COLOR_ANSI_COMP: usize = 13;

#[derive(Debug)]
pub struct Question<'b> {
    pub word: &'b Word,
    pub user_answer: String,
    pub answer_correct: bool,
    pub ab: bool,
    pub term_width: f32,
    pub term_height: f32,
    pub left_spacing_word: usize,
    pub left_spacing_answer: usize,
    pub border_size: usize,
    pub left_spacing_border: usize,
    pub lines_filled: usize,
}

impl Question<'_> {
    pub fn ask(&mut self) {
        // border line
        self.lines_filled = 1;
        let border = vec!["="; self.border_size].join("");
        let quest = if self.ab {
            &self.word.lang_a
        } else {
            &self.word.lang_b
        };
        let quest_string = left_pad(quest, self.left_spacing_word, self.term_width);
        print!(
            "{}",
            top_pad(
                "",
                calc_spacing(self.lines_filled, self.term_height),
                self.term_height
            )
        );
        println!("{}", quest_string);
        println!(
            "{}",
            left_pad(&border, self.left_spacing_border, self.term_width)
        );
        print!(
            "{}",
            left_pad("", self.left_spacing_answer, self.term_width)
        );
        if io::stdout().flush().is_ok() {};
        // quest line
        self.lines_filled += (quest_string.len() as f32 / self.term_width).ceil() as usize;
        if io::stdin().read_line(&mut self.user_answer).is_ok() {}
        self.user_answer = self.user_answer.trim().to_string();
        self.left_spacing_answer = calc_spacing(self.user_answer.len(), self.term_width);
        self.lines_filled += (self.user_answer.len() as f32 / self.term_width).ceil() as usize;
    }

    pub fn check(&mut self) {
        // border line
        self.lines_filled = 1;
        let (quest, quest_anser) = if self.ab {
            (&self.word.lang_a, &self.word.lang_b)
        } else {
            (&self.word.lang_b, &self.word.lang_a)
        };
        self.answer_correct = *quest_anser == self.user_answer;
        let answer_len = self.user_answer.len();
        let (border, answer) = if self.answer_correct {
            (
                Fixed(GREEN)
                    .paint(vec!["|"; self.border_size].join(""))
                    .to_string(),
                Fixed(GREEN).paint(self.user_answer.to_string()),
            )
        } else {
            (
                Fixed(RED)
                    .paint(vec!["x"; self.border_size].join(""))
                    .to_string(),
                Fixed(RED).paint(self.user_answer.to_string()),
            )
        };
        let mut quest_string = quest.to_string();
        if !self.answer_correct {
            quest_string.push_str(&format!(" = {}", quest_anser))
        }
        quest_string = left_pad(
            &quest_string,
            calc_spacing(quest_string.len(), self.term_width),
            self.term_width,
        );
        // quest_anser line
        self.lines_filled += (quest_string.len() as f32 / self.term_width).ceil() as usize;
        self.lines_filled += (self.user_answer.len() as f32 / self.term_width).ceil() as usize;
        print!(
            "{}",
            top_pad(
                "",
                calc_spacing(self.lines_filled, self.term_height),
                self.term_height
            )
        );
        println!("{}", quest_string);
        println!(
            "{}",
            left_pad(&border, self.left_spacing_border, self.term_width)
        );
        if io::stdout().flush().is_ok() {};
        print!(
            "{}",
            left_pad("", self.left_spacing_answer, self.term_width)
        );
        println!("{}", answer)
    }
    pub fn get_display_stats(&mut self) {
        assert!(
            self.term_width > 0.,
            "Terminal width must be greater than  0"
        );
        self.left_spacing_word = calc_spacing(self.word.lang_a.len(), self.term_width);
        self.left_spacing_answer = calc_spacing(self.word.lang_b.len(), self.term_width);

        self.border_size = (self.term_width / 3.) as usize;
        if self.border_size <= self.word.lang_a.len() {
            self.border_size = self.word.lang_a.len()
        }
        if self.border_size >= self.term_width as usize {
            self.border_size = self.term_width as usize
        }
        self.left_spacing_border = calc_spacing(self.border_size, self.term_width);
    }
    pub fn raw_next_action(&mut self) -> String {
        let question_line = left_pad(
            "Next action ",
            calc_spacing("Next action ".len(), self.term_width),
            self.term_width,
        );
        println!("\n{}", question_line);
        print!(
            "{}",
            left_pad("", calc_spacing(1, self.term_width), self.term_width)
        );
        if io::stdout().flush().is_ok() {};
        // question line
        self.lines_filled += (question_line.len() as f32 / self.term_width).ceil() as usize + 1;
        // the input line
        self.lines_filled += 1;
        let mut next_act: String = String::new();
        if io::stdin().read_line(&mut next_act).is_ok() {}
        next_act
    }
}

pub fn calc_spacing(in_word_len: usize, term_width: f32) -> usize {
    let mut spacing = (term_width / 2. - (in_word_len as f32 / 2.)) as usize;
    if in_word_len + spacing >= term_width as usize {
        spacing = 0
    }
    spacing
}

pub fn top_pad(word: &str, spacing: usize, term_height: f32) -> String {
    let padded = format!("{}{}", vec!["\n"; spacing].join(""), word);
    padded
}

pub fn left_pad(word: &str, spacing: usize, term_width: f32) -> String {
    let padded = format!("{}{}", vec![" "; spacing].join(""), word);
    padded
}
pub fn show_stats(
    unit_desc: &str,
    n_words: usize,
    n_total_words: usize,
    correct: usize,
    wrong: usize,
    term_width: f32,
) -> usize {
    let stat_string = format!(
        "Word {} of {} {} ✅{} ❌{}",
        n_words,
        n_total_words,
        unit_desc,
        Fixed(GREEN).paint(correct.to_string()),
        Fixed(RED).paint(wrong.to_string())
    );
    let stat_string_display_size = stat_string.len() - SINGLE_COLOR_ANSI_COMP * 2;
    let lines = (stat_string_display_size as f32 / term_width).ceil() as usize;
    println!(
        "{}",
        left_pad(
            &stat_string,
            calc_spacing(stat_string_display_size, term_width),
            term_width
        )
    );
    lines
}

pub fn clear_screen() {
    print!("\x1Bc");
    if io::stdout().flush().is_ok() {};
    // print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
