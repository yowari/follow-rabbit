use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

use crypto::md5::Md5;
use crypto::digest::Digest;

use rayon::prelude::*;

use word::Letter;

/// Configuration used to search for the anagrams
pub struct AnagramFinder<'a> {
    /// head of the trie of `Letter`s
    head: &'a Letter,
    /// the original phrase to search for anagram
    phrase: &'a str,
    /// list of the accepted hashes
    hashes: &'a [&'a str],
    /// maximum words in an angram
    max_words: usize,
}

impl<'a> AnagramFinder<'a> {

    /// Contructs and configure the search of anagrams.
    pub fn new(head: &'a Letter, phrase: &'a str, hashes: &'a [&'a str], max_words: usize) -> AnagramFinder<'a> {
        AnagramFinder {
            head: head,
            phrase: phrase,
            hashes: hashes,
            max_words: max_words,
        }
    }

    /// Verify if letters in `input` are contained in a `phrase`.
    pub fn contain_str(input: &str, phrase: &str) -> bool {
        let mut word = String::from(phrase);

        for c in input.chars() {
            if !c.is_alphabetic() {
                continue;
            }

            let index = word.find(c);

            match index {
                Some(index) => word.remove(index),
                None        => return false,
            };
        }

        true
    }

    /// Verfies if letters are contained in a `phrase`.
    /// 
    /// Returns a clone of the phrase without the contained `character` or
    /// `None` when the `character` isn't contained.
    pub fn contain_char(character: char, phrase: &String) -> Option<String> {
        let mut remaining_phrase = phrase.clone();

        if !character.is_alphabetic() {
            return Some(remaining_phrase);
        }

        let index = phrase.find(character);

        match index {
            Some(index) => {
                remaining_phrase.remove(index);
                Some(remaining_phrase)
            },
            None => None,
        }
    }

    /// Search for the anagrams having one of the specified hashes.
    /// 
    /// The method communicates the results using a channel (`Receiver`). Each
    /// resulted anagram is accompanied with its hash in the form of a tuple
    /// `(anagram, hash)`.
    pub fn search(&mut self) -> Receiver<(String, String)> {
        let (sender, receiver) = mpsc::channel();

        // remove whitespaces
        let phrase = self.phrase.chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        self.combine(sender, self.head, &String::new(), &phrase);
        
        receiver
    }

    /// The recursive method `combine` construct the anagram by adding one
    /// character to the prefix, `verify` the hash of the candidate and pushes
    /// the result to a channel using the `Sender`.
    /// 
    /// When the candidate is not long enought, meaning that we don't consume
    /// all letters of the phrase, the method calls recursivly `combine` with
    /// the candidate as the prefix and the current node as the parent.
    /// 
    /// # Arguments
    /// 
    /// * `sender` - Anagrams are send through the channel using the `Sender`
    /// * `parent` - Previous `Letter` containing the last accepted character for the current word
    /// * `prefix` - Accepted String. Can also be seen as the path when traversing the tree
    /// * `phrase` - Remaining characters that are not consumed
    fn combine(&self, sender: Sender<(String, String)>, parent: &Letter, prefix: &String, phrase: &String) {
        parent.children().par_iter()
            .for_each_with(sender, |sender, node| {
                match Self::contain_char(node.character(), phrase) {
                    // a character can be consumed
                    Some(remaining_phrase) => {
                        // add the character to construct the anagram candidate
                        // ~ push it to the limit
                        let mut candidate = prefix.clone();
                        candidate.push(node.character());

                        if remaining_phrase.is_empty() && node.is_word() {
                            // verify hash of the candidate is one of hashes list
                            match self.verify(&candidate) {
                                Some(hash) => {
                                    // Yay! found an anagram.
                                    sender.send((candidate, hash)).unwrap();
                                },
                                None => (),
                            }
                        } else if !remaining_phrase.is_empty() {
                            // ~ Open up the limit
                            // ~ Past the point of no return
                            // ~ You've reached the top but still you gotta learn
                            // ~ How to keep it
                            self.combine(sender.clone(), node, &candidate, &remaining_phrase);

                            let word_count = candidate.split_whitespace().count();

                            // ~ Welcome to the limit
                            if node.is_word() && word_count < self.max_words {
                                // ~ Take it baby one step more
                                candidate.push(' ');
                                // ~ The power game's still playing so
                                // ~ You better win it
                                self.combine(sender.clone(), self.head, &candidate, &remaining_phrase);
                            }
                        }
                    },
                    // ~ Don't look down just keep your head
                    // ~ Or you'll be finished
                    None => (),
                };
            });
    }

    /// Verify is the hash of the phrase is one of the provided hashes.
    /// 
    /// Returns the hash `String` if found or `None` if not.
    fn verify(&self, phrase: &str) -> Option<String> {
        let mut hasher = Md5::new();

        hasher.input(phrase.as_bytes());

        let result = hasher.result_str();

        if self.hashes.iter().any(|&hash| result == hash) {
            Some(result)
        } else {
            None
        }
    }

}

#[cfg(test)]
mod test_contain_str {
    use super::*;

    #[test]
    fn test_empty_phrase_input() {
        assert!(AnagramFinder::contain_str("", ""));
    }

    #[test]
    fn test_phrase_without_spaces() {
        // Is MIPS simple? It should...
        assert!(AnagramFinder::contain_str("mips", "simple"));
        // ARM seems to be hard :°
        assert!(!AnagramFinder::contain_str("arm", "hard"));
    }

    #[test]
    fn test_phrase_with_spaces() {
        // positive response: it's a yes for my "dear"
        assert!(AnagramFinder::contain_str("dear", "phrase used here"));
        // negative response: it's a no for my "potatoe" :(
        assert!(!AnagramFinder::contain_str("potatoe", "another phrase"));
    }
}
