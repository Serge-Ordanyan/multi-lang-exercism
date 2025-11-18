pub fn recite(start_bottles: u32, take_down: u32) -> String {
    (0..take_down)
        .map(|i| verse(start_bottles - i))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn verse(n: u32) -> String {
    match n {
        0 => String::new(),
        1 => format!(
            "One green bottle hanging on the wall,\n\
             One green bottle hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be no green bottles hanging on the wall."
        ),
        _ => format!(
            "{} green bottles hanging on the wall,\n\
             {} green bottles hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {} green bottles hanging on the wall.",
            num_to_word(n),                    // first line capitalized
            num_to_word(n),                    // second line capitalized
            num_to_word(n - 1).to_lowercase()  // “there'll be ...” lowercase
        ),
    }
}

fn num_to_word(n: u32) -> String {
    match n {
        0 => "no".to_string(),
        1 => "One".to_string(),
        2 => "Two".to_string(),
        3 => "Three".to_string(),
        4 => "Four".to_string(),
        5 => "Five".to_string(),
        6 => "Six".to_string(),
        7 => "Seven".to_string(),
        8 => "Eight".to_string(),
        9 => "Nine".to_string(),
        10 => "Ten".to_string(),
        _ => n.to_string(),
    }
}
