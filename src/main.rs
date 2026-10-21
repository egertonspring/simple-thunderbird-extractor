use std::{env::{self}, fs::{self, File}, {io::{BufReader, BufRead}}};

fn split_into_single_mails(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("Expected file, got no file!");
    let reader = BufReader::new(file);
    let mut result_string = String::new();
    for line in reader.lines() {
        match &line {
            Ok(line_string) => {
                result_string.push_str(line_string);
                result_string.push_str("\n");
            },
            Err(_) => (),
        }
    }
    let collection = result_string.split("From ").map(String::from).collect();
    collection
}

fn write_mail_file(filename: &str, content: &str) -> std::io::Result<()> {
    fs::write(filename, content)?;
    Ok(())
}

fn restore_mails_from_file(filename: &str) {
    let mails_vector = split_into_single_mails(filename);
    let mut loop_count = 0;
    for mail in mails_vector {
        let mail_filename = format!("mail_{}_{:05}.eml", filename, loop_count);
        let mail_text = format!("From {}", mail);
        let _ = write_mail_file(&mail_filename, &mail_text);
        loop_count+=1;
    }
}

fn main () {
    let args: Vec<String> = env::args().collect();
    let _ = restore_mails_from_file(&args[1]);
}