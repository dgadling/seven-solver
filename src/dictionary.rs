use indicatif::{ProgressBar, ProgressStyle};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Dictionary {
    word_cache: HashSet<String>,
    path_cache: HashSet<String>,
    hashed: HashMap<String, Vec<String>>,
}

impl Dictionary {
    pub fn new(dict_path: &str, min_word_length: usize, quiet: bool) -> Self {
        let mut d = Dictionary {
            word_cache: HashSet::new(),
            path_cache: HashSet::new(),
            hashed: HashMap::new(),
        };

        if !quiet {
            println!("Reading words from {}", &dict_path);
        }

        // NOTE: The only reason we're doing this in two steps is so that we can
        // have the ProgressBar. If we ever decide we don't want that this can
        // all happen with one long chain.
        let words =
            BufReader::new(File::open(&dict_path).unwrap_or_else(|e| {
                panic!("Couldn't read word list from {}: {}", &dict_path, e)
            }))
            .lines()
            .map(|l| l.unwrap())
            .filter(|w| w.len() >= min_word_length)
            .collect::<Vec<String>>();

        let bar: ProgressBar;

        if quiet {
            bar = ProgressBar::hidden();
        } else {
            bar = ProgressBar::new(words.len() as u64);
        }

        bar.set_style(
            ProgressStyle::with_template(
                "{msg} {elapsed} {wide_bar:.blue} {human_pos:>}/{human_len}",
            )
            .unwrap()
            .progress_chars("-> "),
        );

        bar.set_message("Populating caches");

        words.iter().for_each(|word| {
            for l in 2..=word.len() {
                let mut prefix = String::with_capacity(l);
                prefix.push_str(&word[0..l]);
                d.path_cache.insert(prefix);
            }
            d.word_cache.insert(word.clone());

            let mut h_key_parts = word.chars().collect::<Vec<char>>();
            h_key_parts.sort();
            let h_key = String::from_iter(h_key_parts);

            d.hashed
                .entry(h_key)
                .and_modify(|w| w.push(word.to_string()))
                .or_insert(vec![word.to_string()]);
            bar.inc(1);
        });
        bar.finish();

        d
    }

    pub fn has_path(&self, prefix: &str) -> bool {
        let has = self.path_cache.get(prefix);
        has.is_some()
    }

    pub fn is_word(&self, prefix: &str) -> bool {
        let has = self.word_cache.get(prefix);
        has.is_some()
    }

    pub fn make_words_from(&self, letters: &str) -> Option<&Vec<String>> {
        self.hashed.get(letters)
    }
}
