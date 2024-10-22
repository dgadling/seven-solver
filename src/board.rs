use chrono::Local;
use core::fmt;
use serde::{Deserialize, Serialize};
use serde_json::{from_reader, to_writer};
use std::fs::File;

use crate::dictionary::Dictionary;

#[derive(Serialize, Deserialize, Debug)]
pub struct Word {
    path: [(usize, usize); 7],
    word: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    columns: [[char; 7]; 7],
    column_bottom: [usize; 7],
    date: String,
    words: Vec<Word>,
    score: u32,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "lmao")
    }
}

impl Board {
    pub fn get_todays_board() -> Self {
        let today = Local::now().format("%Y%m%d").to_string();
        Self::get_board_for(&today)
    }

    pub fn get_board_for(target_date: &str) -> Self {
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
            date: target_date.to_string(),
            words: vec![],
            score: 700,
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

    pub fn find_words(&mut self, dict: &Dictionary) -> Vec<String> {
        // First, do we currently have a word at the bottom?
        let curr_letters = (0..7).map(|c| self.get(c, 0).unwrap());

        let curr_string = String::from_iter(curr_letters);
        dict.words_from(&curr_string)
    }
}
