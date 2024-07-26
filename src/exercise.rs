use crate::{access_cbor, clear_screen, files, show_stats, update_cbor, Question};
use std::{path::PathBuf, thread, time};
enum InputAction {
    GoOn,
    Stop,
    ForceQuit,
}
pub fn exercise_unit(term_width: usize, term_height: usize, data_path: PathBuf, unit_desc: &str) {
    let mut word_data = access_cbor(data_path.clone());

    let sr: Vec<f64> = word_data.iter().map(|(_, x)| x.success_rate).collect();
    let mut ids: Vec<usize> = word_data.iter().map(|(&i, _)| i).collect();
    ids.sort_by(|&a, &b| sr[a].partial_cmp(&sr[b]).unwrap());

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
            term_width as f32,
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
            term_width: term_width as f32,
            term_height: term_height as f32,
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
            InputAction::GoOn => continue,
            InputAction::Stop => break,
            InputAction::ForceQuit => return,
        }
    }

    clear_screen();
    show_stats(
        unit_desc,
        n_total_words,
        n_total_words,
        n_correct,
        n_wrong,
        term_width as f32,
    );
    update_cbor(data_path, &word_data);
}
fn convert_commands(inp: &str) -> InputAction {
    let action = match inp {
        ":q" => InputAction::Stop,
        ":fq" => InputAction::ForceQuit,
        "" | _ => InputAction::GoOn,
    };
    action
}
