use std::fs;

pub fn read_input_to_file(input_f: &str) -> fs::File {
    let input_f = std::path::Path::new(input_f);
    match std::fs::File::open(input_f) {
        Ok(file) => file,
        Err(e) => panic!("could not open file {}: {}!", input_f.display(), e),
    }
}
