pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}




// oteher solution variants
// pub fn reverse(input: &str) -> String {
//     if input.is_empty() {
//         String::new()
//     } else {
//         let mut chars: Vec<char> = input.chars().collect();
//         let last_char = chars.pop().unwrap();
//         format!("{}{}", reverse(&chars.iter().collect::<String>()), last_char)
//     }
// }



// second
// pub fn reverse(input: &str) -> String {
//     if input.is_empty() {
//         String::new()
//     } else {
//         let mut chars: Vec<char> = input.chars().collect();
//         let last_char = chars.pop().unwrap();
//         format!("{}{}", reverse(&chars.iter().collect::<String>()), last_char)
//     }
// }
