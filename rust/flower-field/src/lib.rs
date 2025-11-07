pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    if rows == 0 {
        return vec![];
    }
    let cols = garden[0].len();

    // Convert input garden to a Vec of & [u8] for byte-level access (ASCII)
    let garden_bytes: Vec<&[u8]> = garden.iter()
        .map(|row| row.as_bytes())
        .collect();

    let mut result: Vec<String> = Vec::with_capacity(rows);

    for i in 0..rows {
        // Create a byte buffer for this row
        let mut row_buf: Vec<u8> = Vec::with_capacity(cols);

        for j in 0..cols {
            let cell = garden_bytes[i][j];
            if cell == b'*' {
                // Flower, keep as '*'
                row_buf.push(b'*');
            } else {
                // Empty space, count adjacent flowers
                let mut count = 0;
                // iterate −1, 0, +1 for row and col offsets
                for di in -1i32..=1 {
                    for dj in -1i32..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        // target row, col
                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;
                        if ni >= 0 && (ni as usize) < rows && nj >= 0 && (nj as usize) < cols {
                            if garden_bytes[ni as usize][nj as usize] == b'*' {
                                count += 1;
                            }
                        }
