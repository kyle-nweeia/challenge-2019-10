use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::time;
use hashbrown::HashMap;

#[derive(Debug)]
#[derive(Clone)]
struct Word {
    word: String
}

impl Word{
    fn new(word: String) -> Word {
        Word{ word: word }
    }

    fn len(&self) -> usize {
        self.word.len()
    }

    fn score(&self) -> isize {
        let values = vec![1, 3, 3, 2, 1, 4,
                            2, 4, 1, 8, 5, 1,
                            3, 1, 1, 3, 10, 1,
                            1, 1, 1, 4, 4, 8, 4,
                            10];
        let mut value: isize = 0;
        for c in self.word.chars() {
            value += values[c as usize - 97];
        }
        value
    }

    fn decompose(&self) -> HashMap<char, isize> {
        let mut decomposed = HashMap::new();
        for c in self.word.chars() {
            *decomposed.entry(c).or_insert(0) += 1;
        }
        decomposed
    }

    fn compare(&self, mut compared: HashMap<char, isize>) -> bool {
        let alphabet: String = "abcdefghijklmnopqrstuvwxyz".to_string();
        let mut difference = HashMap::new();
        let mut word = self.decompose();
        for c in alphabet.chars() {
            let diff = *word.entry(c).or_insert(0) as isize -
                *compared.entry(c).or_insert(0) as isize;
            if diff < 0 {
                return false
            }
            *difference.entry(c).or_insert(0) = diff;
        }
        true
    }
}

fn find_best(path: String, tiles: Word) -> Word {
    let mut best = Word::new("a".to_string());
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let word = Word::new(line.unwrap());
        if word.len() <= tiles.len() {
            if word.score() > best.score() {
                if tiles.compare(word.decompose()) {
                    best = word;
                }
            }
        }
    }
    best
}

fn main() {
    let start = time::Instant::now();
    let tiles = Word::new(env::args().nth(1).unwrap().to_string());
    let best = find_best("../data/dictionary.txt".to_string(), tiles);
    let elapsed = start.elapsed();
    let ms = ((elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0)) * 1000.0;
    println!("pard68, Rust, {}, {}, {}, Decomposition", best.word, best.score(), ms)
}

