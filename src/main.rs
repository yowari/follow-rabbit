#![feature(nll)]

extern crate crypto;
extern crate rayon;
extern crate clap;

mod anagram;
mod word;

use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use clap::{Arg, App};

use anagram::AnagramFinder;
use word::build_trie;

fn main() {
    const DEFAULT_OUTPUT_FILE: &str = "anagrams";
    const DEFAULT_DICTIONARY_PATH: &str = "wordlist";
    const DEFAULT_MAX_WORDS: usize = 4;
    const DEFAULT_MIN_WORD_LEN: usize = 2;

    let matches = App::new("Follow the Rabbit and see how deep the hole goes.")
        .version("1.0.0")
        .author("yowari <yowari@outlook.com>")
        .about("Search for anagrams")
        .arg(Arg::with_name("dictionary")
            .short("d")
            .long("dictionary")
            .value_name("FILE")
            .help("Select the dictionary file. Default = wordlist")
            .takes_value(true))
        .arg(Arg::with_name("output file")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Output file path. Default = anagrams")
            .takes_value(true))
        .arg(Arg::with_name("max words")
            .short("w")
            .long("words")
            .value_name("INTEGER")
            .help("Set the maximum number of words in an angram. Default = 4")
            .takes_value(true))
        .arg(Arg::with_name("min word length")
            .short("l")
            .long("length")
            .value_name("INTEGER")
            .help("Set the minimum length of a word. Default = 2")
            .takes_value(true))
        .get_matches();

    let phrase = "poultry outwits ants";
    let hashes = [
        "e4820b45d2277f3844eac66c903e84be",
        "23170acc097c24edb98fc5488ab033fe",
        "665e5bcb0c20062fe8abaaf4628bb154"
    ];

    let output_file = matches.value_of("output file")
        .unwrap_or(DEFAULT_OUTPUT_FILE);
    let dictionary_path = matches.value_of("dictionary")
        .unwrap_or(DEFAULT_DICTIONARY_PATH);

    let max_words = match matches.value_of("max words") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => DEFAULT_MAX_WORDS,
    };

    let min_word_len = match matches.value_of("min word length") {
        Some(s) => s.parse::<usize>().unwrap(),
        None => DEFAULT_MIN_WORD_LEN,
    };

    let instant = Instant::now();

    let mut out = File::create(output_file).unwrap();

    let content = load_file(dictionary_path);
    let words = filter_words(&content, phrase, min_word_len);

    let trie = build_trie(&words);

    let mut finder = AnagramFinder::new(&trie, phrase, &hashes, max_words);
    let result = finder.search();

    for (anagram, hash) in result.iter() {
        writeln!(out, "{} {}", hash, anagram).unwrap();
    }

    println!("executed in: {:?}", instant.elapsed());
}

/// Read dictionary file and returns its content.
fn load_file(path: &str) -> String {
    let mut file = File::open(path)
        .expect("Failed opening wordlist file");

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Something went wrong read the wordlist file");

    content
}

/// Read, sorts, filters and transforms the content of list of words and returns
/// a vector of words.
fn filter_words(content: &String, phrase: &str, min_word_len: usize) -> Vec<String> {
    let mut words = content.lines()
        .filter(|w| !w.is_empty())
        .filter(|w| !w.chars().any(|c| !c.is_alphabetic()))
        .map(|w| w.to_lowercase())
        .collect::<Vec<String>>();

    words.sort();
    words.dedup();
    words = words.into_iter()
        .filter(|word| AnagramFinder::contain_str(word, phrase))
        .filter(|word| word.len() >= min_word_len)
        .collect();

    words.sort_by_key(|w| w.len());

    words
}
