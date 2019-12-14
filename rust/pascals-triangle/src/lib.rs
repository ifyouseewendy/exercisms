pub struct PascalsTriangle(u32);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut v: Vec<Vec<u32>> = vec![];

        for i in 0..self.0 {
            let mut row = vec![];

            if i == 0 {
                row.push(1);
            } else {
                row.push(1);
                for x in v[(i-1) as usize].windows(2).map(|s| s.iter().sum()) {
                    row.push(x)
                }
                row.push(1);
            }

            v.push(row);
        }

        v
    }
}
