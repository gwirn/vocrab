use std::fs;
use std::path::Path;
pub fn get_vocabs(vc_base: &Path) -> Vec<String> {
    let vocab_files = match fs::read_dir(vc_base.join("vocabulary")) {
        Ok(ps) => ps,
        Err(e) => panic!(
            "Can't read vocabulary files in {} because of {}",
            vc_base.join("vocabulary").display(),
            e
        ),
    };
    let vocabs: Vec<String> = vocab_files
        .into_iter()
        .map(|x| match x {
            Ok(xx) => match xx.file_name().to_str() {
                Some(xxx) => xxx.to_string(),
                None => "".to_string(),
            },
            Err(_) => "".to_string(),
        })
        .filter(|x| !x.is_empty() && x.ends_with(".cbor"))
        .collect();
    vocabs
}
