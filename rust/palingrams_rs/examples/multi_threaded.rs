use palingrams_rs::{find_palingrams_concurrent, load_file_data};
use std::time::Instant;

fn main() {
    let word_list = load_file_data("./words.txt");
    let start = Instant::now();
    let result = find_palingrams_concurrent(word_list);
    println!("{:?}", result);
    println!("total palingrams = {}", result.len());
    let duration = start.elapsed();
    println!("Runtime for this program was: {:?}", duration);
}
