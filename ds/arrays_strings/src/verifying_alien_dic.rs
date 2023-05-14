use std::{cmp::min, collections::HashMap};

fn is_words_ordered_lexicology(word1: &str, word2: &str, order_hash: &HashMap<char, usize>) -> bool {
    let min_length = min(word1.len(), word2.len());

    for i in 0..min_length {
        let char1 = word1.chars().nth(i).unwrap();
        let char2 = word2.chars().nth(i).unwrap();

        if char1 == char2 {continue;}

        return order_hash.get(&char1) < order_hash.get(&char2);
    }
    return !(word1.len() > word2.len());
}

#[allow(dead_code)]
fn verify_alien_alpha(_words: Vec<&str>, alphabet: &str) -> bool {
    // create map
    let order_hash = alphabet
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<HashMap<char, usize>>();

    // review order
    for i in 0.._words.len() - 1 {
        let word1 = _words[i];
        let word2 = _words[i + 1];

        if is_words_ordered_lexicology(word1, word2, &order_hash) == false {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn return_true() {
        let words = vec!["habito", "hacer", "lectura", "sonreir"];
        let alphabet = "hlabcdefgijkmnopqrstuvwxyz";

        let result = verify_alien_alpha(words, alphabet);

        assert_eq!(result, true);
    }

    #[test]
    fn return_in_not_order() {
        let words = vec!["habito", "hacer", "sonreir", "lectura"];
        let alphabet = "hlabcdefgijkmnopqrstuvwxyz";

        let result = verify_alien_alpha(words, alphabet);

        assert_eq!(result, false);
    }

    #[test]
    fn return_false() {
        let words = vec!["conocer", "cono"];
        let alphabet: &str = "abdcefghijklmnopqrstuvwxyz";

        let result = verify_alien_alpha(words, alphabet);

        assert_eq!(result, false);
    }
}
