pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() || input[0].is_empty() {
        return Vec::new();
    }

    let row_max = input
        .iter()
        .map(|row| *row.iter().max().unwrap())
        .collect::<Vec<u64>>();
    let col_min = (0..input[0].len())
        .map(|i| input.iter().map(|row| row[i]).min().unwrap())
        .collect::<Vec<u64>>();

    let mut ret = vec![];
    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == row_max[i] && input[i][j] == col_min[j] {
                ret.push((i, j))
            }
        }
    }
    ret
}
