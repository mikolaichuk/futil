use glob::glob;
use std::error::Error;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    recursive_search("/home/mikhail/study", "txt")?;
    Ok(())
}

fn recursive_search(dir: &str, ext: &str) -> Result<(), Box<dyn Error>> {
    let pattern = format!("{}{}{}", dir, "/**/*.", ext);
    for entry in glob(&pattern)? {
        let path = entry?;
        let (lines_amount, body) = get_lines_amount(&path);
        println!(
            "{} lines: {} \n==== Content =====\n{}\n\n",
            path.display(),
            lines_amount,
            body
        );
    }

    Ok(())
}

fn get_lines_amount(path: &PathBuf) -> (usize, String) {
    let file_try = std::fs::File::open(path);
    let file = match file_try {
        Err(_) => return (0, String::from("File can not be opened")),
        Ok(file) => file,
    };
    let body =
        std::fs::read_to_string(path).unwrap_or(String::from("File content can not be printed"));
    let f = BufReader::new(file);
    (f.lines().count(), body)
}
