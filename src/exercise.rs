use crate::{
    access_cbor, calc_spacing, clear_screen, files, left_pad, show_stats, start, update_cbor,
    Question,
};
use core::f32;
use std::io::{self, Write};
use std::{path::PathBuf, process, thread, time};

#[derive(Debug)]
enum InputAction {
    NextWord,
    QuitUnit,
    QuitUnitNoSave,
    QuitTotal,
    QuitTotalNoSave,
}
pub fn exercise_unit(term_width: usize, term_height: usize, data_path: PathBuf, unit_desc: &str) {
    let mut word_data = access_cbor(data_path.clone());
    let t_width = term_width as f32;
    let t_height = term_height as f32;

    let mut ids: Vec<usize> = word_data.iter().map(|(&i, _)| i).collect();
    ids.sort_by(|&a, &b| {
        word_data
            .get(&a)
            .unwrap()
            .success_rate
            .total_cmp(&word_data.get(&b).unwrap().success_rate)
    });

    let n_total_words = ids.len();
    let mut n_correct = 0;
    let mut n_wrong = 0;
    for (ci, i) in ids.iter().enumerate() {
        clear_screen();
        let n_lines_stats = show_stats(
            unit_desc,
            ci + 1,
            n_total_words,
            n_correct,
            n_wrong,
            t_width,
        );
        let i_word: &mut files::Word = match word_data.get_mut(&i) {
            Some(word) => word,
            None => {
                eprintln!("Can't access word with id {}", i);
                let sleep_time = time::Duration::from_secs(5);
                thread::sleep(sleep_time);
                continue;
            }
        };
        let mut q = Question {
            word: &i_word,
            user_answer: "".to_string(),
            answer_correct: false,
            ab: true,
            term_width: t_width,
            term_height: t_height,
            left_spacing_word: 0,
            left_spacing_answer: 0,
            border_size: 0,
            left_spacing_border: 0,
            lines_filled: n_lines_stats,
        };
        q.get_display_stats();
        q.ask();
        clear_screen();
        q.check();
        if q.answer_correct {
            n_correct += 1
        } else {
            n_wrong += 1
        }
        let next_act = convert_commands(q.raw_next_action().trim());
        i_word.update_sucr(q.answer_correct);
        match next_act {
            InputAction::NextWord => continue,
            InputAction::QuitUnit => break,
            InputAction::QuitUnitNoSave => return,
            InputAction::QuitTotalNoSave => {
                clear_screen();
                process::exit(0)
            }
            InputAction::QuitTotal => {
                update_cbor(data_path.clone(), &word_data);
                clear_screen();
                println!("Vocrab saved current progress and quit");
                process::exit(0)
            }
        }
    }

    clear_screen();
    show_stats(
        unit_desc,
        n_total_words,
        n_total_words,
        n_correct,
        n_wrong,
        t_width,
    );
    update_cbor(data_path, &word_data);

    let mut ids: Vec<usize> = word_data.iter().map(|(&i, _)| i).collect();
    ids.sort_by(|&a, &b| {
        word_data
            .get(&a)
            .unwrap()
            .success_rate
            .total_cmp(&word_data.get(&b).unwrap().success_rate)
    });
    for i in &ids[..{
        if ids.len() < 10 {
            ids.len()
        } else {
            10
        }
    }] {
        let i_word: &mut files::Word = match word_data.get_mut(&i) {
            Some(word) => word,
            None => {
                continue;
            }
        };
        let out_line = format!("{} = {} ", i_word.lang_a, i_word.lang_b);
        println!(
            "{}",
            left_pad(&out_line, calc_spacing(out_line.len(), t_width), t_width)
        )
    }

    let question_line = left_pad(
        "Next action ",
        calc_spacing("Next action ".len(), t_width),
        t_width,
    );
    println!("\n{}", question_line);
    print!("{}", left_pad("", calc_spacing(1, t_width), t_width));
    if io::stdout().flush().is_ok() {};
    let mut act_string: String = String::new();
    if io::stdin().read_line(&mut act_string).is_ok() {}
    let next_act = convert_commands(act_string.trim());
    match next_act {
        InputAction::QuitTotal => {
            clear_screen();
            process::exit(0)
        }
        _ => {}
    }

    start()
}

fn convert_commands(inp: &str) -> InputAction {
    let action = match inp {
        ":q" => InputAction::QuitUnit,
        ":fqu" => InputAction::QuitUnitNoSave,
        ":qa" => InputAction::QuitTotal,
        ":fqa" => InputAction::QuitTotalNoSave,
        "" | _ => InputAction::NextWord,
    };
    action
}
