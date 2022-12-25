use std::{
    collections::{HashMap, HashSet, VecDeque},
    io,
};

fn main() -> io::Result<()> {
    for line in io::stdin().lines() {
        let mut last_chars = std::collections::VecDeque::<char>::new();
        let line = line.unwrap();
        let mut chars = line.char_indices();
        for c in chars {
            let (i, c) = c;
            last_chars.push_back(c);
            if last_chars.len() < 14 {
                continue;
            }
            if is_unique(&last_chars) {
                println!("{}", i + 1);
                break;
            } else {
                last_chars.pop_front();
            }
        }
    }
    Ok(())
}

fn is_unique(chars: &VecDeque<char>) -> bool {
    let mut hash = HashSet::new();
    for c in chars {
        if hash.contains(c) {
            return false;
        } else {
            hash.insert(c);
        }
    }
    true
}
