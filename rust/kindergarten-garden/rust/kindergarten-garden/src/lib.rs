const STUDENTS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let rows: Vec<Vec<char>> = diagram.lines().map(|line| line.chars().collect()).collect();

    let index = STUDENTS
        .iter()
        .position(|&s| s == student)
        .expect("unknown student");

    let start = index * 2;

    rows.iter()
        .flat_map(|row| &row[start..start + 2])
        .map(|&c| match c {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            other => panic!("unknown plant code: {other}"),
        })
        .collect()
}
