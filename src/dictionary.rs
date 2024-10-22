use fast_glob::glob_match;
//use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Dictionary {
    words: HashSet<String>,
    word_length: usize,
}

impl Dictionary {
    pub fn new(dict_path: &str, word_length: usize) -> Self {
        // NOTE: The only reason we're doing this in two steps is so that we can
        // have the ProgressBar. If we ever decide we don't want that this can
        // all happen with one long chain.
        let words = BufReader::new(
            File::open(&dict_path)
                .unwrap_or_else(|e| panic!("Couldn't read word list from {}: {}", &dict_path, e)),
        )
        .lines()
        .map(|l| l.unwrap())
        .filter(|w| w.len() >= word_length)
        .collect::<Vec<String>>();

        let mut d = Dictionary {
            words: HashSet::with_capacity(words.len()),
            word_length,
        };

        words.iter().for_each(|word| {
            d.words.insert(word.to_string());
        });

        d
    }

    // Technically could make these two a trait or enum or something, idk.
    // Having the finder variable below be a little ugly is OK for now
    fn find_with_glob(&self, chars: Vec<char>) -> Vec<String> {
        let glob = String::from_iter(chars);

        self.words
            .iter()
            .filter(|w| glob_match(&glob, w))
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    }

    fn find_plain(&self, chars: Vec<char>) -> Vec<String> {
        let s = String::from_iter(chars);
        if self.words.contains(&s) {
            vec![s]
        } else {
            vec![]
        }
    }

    pub fn words_from(&self, characters: &str) -> Vec<String> {
        let finder: fn(&Dictionary, Vec<char>) -> Vec<String>;

        let glob_swapped = characters.replace("*", "?");

        if glob_swapped.contains("?") {
            finder = Dictionary::find_with_glob;
        } else {
            finder = Dictionary::find_plain;
        }

        glob_swapped
            .chars()
            .permutations(self.word_length)
            .map(|chars| finder(self, chars))
            .flatten()
            .collect::<HashSet<String>>()
            .into_iter()
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_letters_only() {
        let d = Dictionary::new("resources/7x7-word-list-sevens-only.txt", 7);
        let input_letters = "csiplek";

        assert_eq!(d.words_from(input_letters), vec!["pickles"]);
    }

    #[test]
    fn test_one_wildcard() {
        let d = Dictionary::new("resources/7x7-word-list-sevens-only.txt", 7);
        let input_letters = "csipl?k";

        assert_eq!(d.words_from(input_letters), vec!["pickles"]);
    }
}
