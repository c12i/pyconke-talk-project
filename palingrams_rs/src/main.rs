#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

fn load_file_data(path: &'static str) -> Vec<String> {
    let content = std::fs::read_to_string(path).unwrap();
    content
        .lines()
        .map(|l| String::from(l).to_ascii_lowercase())
        .collect::<Vec<String>>()
}

fn find_palingrams(word_list: Vec<String>) -> Vec<(String, String)> {
    let mut palingram_list = Vec::new();
    for word in word_list.iter() {
        let word_len = word.len();
        let word_reversed = word.chars().rev().collect::<String>();
        if word_len > 1 {
            for i in 0..word_len {
                if word[i..] == word_reversed[..word_len - i]
                    && word_list.contains(&word_reversed[word_len - i..].to_owned())
                {
                    palingram_list
                        .push((word.to_owned(), word_reversed[word_len - i..].to_owned()));
                }
                if word[..i] == word_reversed[word_len - i..]
                    && word_list.contains(&word_reversed[..word_len - i].to_owned())
                {
                    palingram_list
                        .push((word.to_owned(), word_reversed[..word_len - i].to_owned()));
                }
            }
        }
    }
    palingram_list
}

fn find_palingrams_concurrent(word_list: Vec<String>) -> Vec<(String, String)> {
    let word_list = Arc::new(word_list);
    let chunks = word_list
        .chunks(word_list.len() / num_cpus::get())
        .map(|v| v.to_vec())
        .collect::<Vec<Vec<_>>>();
    let mut handles = Vec::with_capacity(chunks.len());
    let palingram_list = Arc::new(Mutex::new(Vec::new()));
    for c in chunks {
        let source_word_list = word_list.clone();
        let palingram_list = palingram_list.clone();
        handles.push(thread::spawn(move || {
            for word in c.iter() {
                let word_len = word.len();
                let word_reversed = word.chars().rev().collect::<String>();
                if word_len > 1 {
                    for i in 0..word_len {
                        if word[i..] == word_reversed[..word_len - i]
                            && source_word_list.contains(&word_reversed[word_len - i..].to_owned())
                        {
                            palingram_list
                                .lock()
                                .unwrap()
                                .push((word.to_owned(), word_reversed[word_len - i..].to_owned()));
                        }
                        if word[..i] == word_reversed[word_len - i..]
                            && source_word_list.contains(&word_reversed[..word_len - i].to_owned())
                        {
                            palingram_list
                                .lock()
                                .unwrap()
                                .push((word.to_owned(), word_reversed[..word_len - i].to_owned()));
                        }
                    }
                }
            }
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
    let result = palingram_list.lock().unwrap();
    result.to_vec()
}

fn main() {
    let word_list = load_file_data("words.txt");
    let start = Instant::now();
    let result = find_palingrams_concurrent(word_list);
    // let result = find_palingrams(word_list);
    println!("{:?}", result);
    println!("total palingrams = {}", result.len());
    let duration = start.elapsed();
    println!("Runtime for this program was: {:?}", duration);
}
