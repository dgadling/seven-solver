use chrono::Local;
use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};
use std::fs::File;

use crate::dictionary::Dictionary;

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    columns: [[char; 7]; 7],
    column_bottom: [usize; 7],
    date: String,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lmao")
    }
}

impl Board {
    pub fn get_todays_board() -> Self {
        let today = Local::now().format("%Y%m%d").to_string();
        Self::get_board_for(today)
    }

    pub fn get_board_for(target_date: String) -> Self {
        let target = format!("boards/{}.json", target_date);
        let target_f = std::path::Path::new(&target);
        if target_f.exists() {
            let b: Board = from_reader(File::open(&target_f).unwrap()).unwrap();
            return b;
        }

        let res =
            reqwest::blocking::get(format!("https://7x7.game/games/{}.json", target_date)).unwrap();
        let letters = res.json::<Vec<String>>().unwrap();

        let mut columns = [[' '; 7]; 7];

        let mut curr_row = 0;
        for row in letters.iter().rev() {
            for (idx, c) in row.chars().enumerate() {
                columns[idx][curr_row] = c.to_ascii_lowercase();
            }
            curr_row += 1;
        }

        let b = Board {
            columns,
            column_bottom: [0, 0, 0, 0, 0, 0, 0],
            date: target_date,
        };

        let out_f = File::create(target).unwrap();
        to_writer(out_f, &b).unwrap();
        b
    }

    pub fn get(&self, col: usize, offset: usize) -> Option<char> {
        let r = self.column_bottom[col] + offset;
        if r >= 7 {
            return None;
        }
        Some(self.columns[col][r])
    }

    pub fn find_words(&self, dict: &Dictionary) -> Vec<String> {
        // First, do we currently have a word?
        let mut curr_letters = (0..6)
            .map(|c| self.get(c, 0).unwrap())
            .collect::<Vec<char>>();
        curr_letters.sort();
        let potential_word = String::from_iter(curr_letters);

        let made_words = dict.make_words_from(&potential_word);
        if made_words.is_none() {
            return vec![];
        }

        return made_words.unwrap().clone();
    }
}
