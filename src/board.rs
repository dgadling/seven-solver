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

    pub fn find_word(&self, dict: &Dictionary) -> String {
        println!("Looking for a word");
        for col in 0..6 {
            println!(
                "Starting @ {} found {}",
                col,
                self.find_word_starting_from(col, dict)
            );
        }

        "I'll find something to put here".to_string()
    }

    fn find_word_starting_from(&self, col: usize, dict: &Dictionary) -> Option<String> {
        let c = self.get(col, 0);
        if c.is_none() {
            return None;
        }

        let c = c.unwrap().to_string();
        println!("  checking {}", c);
        if dict.has_path(&c) {
            println!("Ok, can start with {}", c);
        }
    }
}
