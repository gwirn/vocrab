use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            123456789";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Word {
    pub lang_a: String,
    pub lang_b: String,
    pub success: i64,
    pub failed: i64,
    pub success_rate: f64,
}
impl Word {
    pub fn update_sucr(&mut self, correct: bool) {
        if correct {
            self.success += 1;
        } else {
            self.failed += 1;
        }
        if self.failed == 0 {
            self.success_rate = 1.0;
        } else if self.success == 0 {
            self.success_rate = 0.;
        } else {
            self.success_rate = self.success as f64 / (self.failed as f64 + self.success as f64)
        }
    }
}

pub fn csv_2_cbor(path: &str, cbor_path: &str) {
    let file = File::open(path).expect("Couldn't read map file");
    let buffer = BufReader::new(file).lines();
    let mut words = HashMap::new();
    for (ci, i) in buffer.enumerate() {
        let line = match i {
            Ok(l) => l,
            Err(e) => {
                eprintln!("{:?}", e);
                eprintln!("Cannot read line [{}] - skipping...", ci);
                continue;
            }
        };
        let split_line: Vec<_> = line.trim().split(',').collect();
        assert!(
            split_line.len() == 2,
            "line {} '{}' is split into {} parts instead of 2",
            ci,
            line,
            split_line.len()
        );
        let conv_line = Word {
            lang_a: split_line[0].trim().to_string(),
            lang_b: split_line[1].trim().to_string(),
            success: 0,
            failed: 0,
            success_rate: 0.,
        };
        words.insert(ci, conv_line);
    }
    let word_file = File::create(cbor_path).expect("Unable to create cbor file ");
    serde_cbor::to_writer(word_file, &words).expect("Unable to serialize HashMap");
}

pub fn update_cbor(cbor_path: PathBuf, word_map: &HashMap<usize, Word>) {
    let tmp_path = tmp_filepath(cbor_path.clone());
    let tmppathbind = tmp_path.clone();
    let tmp_path_disp = tmppathbind.to_str().unwrap_or("Can't display tmp path");

    if Path::new(&cbor_path).exists() {
        fs::rename(cbor_path.clone(), tmp_path.clone())
            .expect("Can't create temp file to update cbor file")
    };
    let word_file = match File::create(cbor_path.clone()) {
        Ok(wf) => wf,
        Err(e) => {
            match fs::rename(tmp_path, cbor_path.clone()) {
                Ok(_) => {}
                Err(_) => eprintln!(
                    "Unable to recreate cbor file from tmp file {} now saved as {}",
                    tmp_path_disp,
                    cbor_path.display()
                ),
            };
            panic!(
                "While trying to create the new cbor file following error occurred {}",
                e
            )
        }
    };

    match serde_cbor::to_writer(word_file, word_map) {
        Ok(_) => {}
        Err(e) => {
            match fs::rename(tmp_path, cbor_path.clone()) {
                Ok(_) => {}
                Err(_) => eprintln!(
                    "Unable to recreate cbor file from tmp file {} now saved as {}",
                    tmp_path_disp,
                    cbor_path.display()
                ),
            };
            panic!(
                "While trying to write to the new cbor file following error occurred {}",
                e
            )
        }
    };
    match fs::remove_file(tmp_path) {
        Ok(_) => {}
        Err(e) => eprintln!("Unable to remove tmp file at {}", tmp_path_disp),
    }
}

pub fn access_cbor(cbor_path: PathBuf) -> HashMap<usize, Word> {
    let vocab_file = File::open(cbor_path).expect("Unable to open cbor file");
    let vocab: HashMap<usize, Word> =
        serde_cbor::from_reader(vocab_file).expect("Unable to create HashMap from cbor file");
    vocab
}

fn tmp_filepath(current_path: PathBuf) -> PathBuf {
    let mut rng = rand::thread_rng();
    let tfile_add = 16;
    let add: String = (0..tfile_add)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    let tmp_path = Path::new(&current_path)
        .parent()
        .expect("Unable to get the current file path")
        .join(format!("tmp_{}.cbor", add));
    tmp_path
}
