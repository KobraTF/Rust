use std::io;

fn main() {
    let letters = "qwertyuiopasdfghjklzxcvbnmQWERTYUIOPASDFGHJKLZXCVBNM";
    let less_letters = "aeouiyAEUIOY";

    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line.");

    sentence = sentence.trim().parse().expect("err");
    let words: Vec<_> = sentence.split(" ").collect();
    let mut new_words = String::new();

    for word in words {
        new_words = new_words + &pig_latin(&letters, &less_letters, word) + " ";
    };
    println!("{new_words}");
    
}

fn pig_latin (letters:&str, less_letters:&str, word:&str) -> String {
    if letters.find(&word[0..1]).is_some() {
        let mut new_word = String::new();
        if less_letters.find(&word[0..1]).is_some() {
            println!("Yes");
            new_word = new_word + &word;
            new_word = new_word + &"-hay";
            return new_word
        } else {
            println!("No");
            new_word = new_word + &word[1..];
            new_word = new_word + &"-";
            new_word = new_word + &word[0..1];
            new_word = new_word + &"ay";
            return new_word
        }
    };
    return String::from(word)  
}
