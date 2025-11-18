pub fn recite(start_bottles: u32, take_down: u32) -> String {
    fn verse(n: u32) -> String {
        match n {
            0 => String::new(),
            1 => String::from(
                "One green bottle hanging on the wall,\n\
                 One green bottle hanging on the wall,\n\
                 And if one green bottle should accidentally fall,\n\
                 There'll be no green bottles hanging on the wall.",
            ),
            _ => format!(
                "{n} green bottles hanging on the wall,\n\
                 {n} green bottles hanging on the wall,\n\
                 And if one green bottle should accidentally fall,\n\
                 There'll be {} green bottles hanging on the wall.",
                n - 1
            ),
        }
    }

    (0..take_down)
        .map(|i| verse(start_bottles - i))
        .collect::<Vec<_>>()
        .join("\n\n")
}
