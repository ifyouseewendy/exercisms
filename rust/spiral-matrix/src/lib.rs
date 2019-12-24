pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 { return Vec::new() }

    let mut v = vec![vec![0; size as usize]; size as usize];

    let mut position = (0, 0);
    let mut direction = Direction::East;
    let mut num = 1;

    loop {
        let (x, y) = position;
        v[x][y] = num;

        num += 1;
        if num > size * size { break; }

        for _ in 0..2 {
            match direction {
                Direction::East     if y + 1 < size as usize && v[x][y+1] == 0  => { position = (x, y+1); break; },
                Direction::South    if x + 1 < size as usize && v[x+1][y] == 0  => { position = (x+1, y); break; },
                Direction::West     if y >= 1 && v[x][y-1] == 0                 => { position = (x, y-1); break; },
                Direction::North    if x >= 1 && v[x-1][y] == 0                 => { position = (x-1, y); break; },
                _ => direction = Direction::next(direction),
            }
        }
    }

    v
}

enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    pub fn next(d: Direction) -> Self {
        match d {
            Direction::East  => Direction::South,
            Direction::South => Direction::West,
            Direction::West  => Direction::North,
            Direction::North => Direction::East,
        }
    }
}
