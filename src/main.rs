use std::{fs, io::Write};

fn main() {
    // Get file path with console line arguments.
    let file_path = std::env::args().nth(1).expect("no path given");
    let output_path = std::env::args().nth(2).expect("no output path given");
    //println!("In file {}", file_path);
    println!("CHECK: ARGS");

    // Get file content data.
    let contents = fs::read_to_string(file_path.clone()).expect("should have been able to read the file");
    //println!("File Content:\n{contents}");
    println!("CHECK: READ CONTENT");

    // Convert content data (string) to char array.
    let content_chars: Vec<_> = contents.chars().collect();
    let content_chars_reversed: Vec<_> = content_chars.clone().into_iter().rev().collect();
    //println!("Char List:\n{:?}", content_chars);
    println!("CHECK: CONTENT CHAR SET");

    // Convert char array to u32 array for sett ascii values to ascii_content (u32) Vector.
    let mut ascii_content: Vec<String> = Vec::new();
    for i in content_chars.iter() {
        let ascii_value = *i as u32;
        let mut push_value: String = String::new();
        let ascii_value_str = ascii_value.to_string();

        if i != &content_chars_reversed[0] {
            push_value = ascii_value_str + " ";
        } else {
            push_value = ascii_value_str;
        }
        ascii_content.push(push_value);
    }
    //println!("Ascii List:\n{:?}", ascii_content);
    println!("CHECK: CONTENT CHAR SET TO ASCII SET");

    // Create a new file and write it to the new ascii values.
    let file_path_splitted: Vec<_> = file_path.split("\\").collect();
    let file_path_reversed: Vec<_> = file_path_splitted.clone().into_iter().rev().collect();
    println!("CHECK: SPLIT THE FILE PATH {:?}\nREVERSED: {:?}", file_path_splitted, file_path_reversed);
    let mut file = std::fs::File::create(format!("{}\\e.{}", output_path, file_path_reversed[0])).expect("file could not be created");
    println!("CHECK: FILE CREATED TO THE OUTPUT PATH");
    for i in ascii_content.iter_mut() {
        file.write(&i.to_string().as_bytes()).expect("Can't write to file");
    }
    println!("CHECK: WRITTEN INTO THE OUTPUT FILE");
}
