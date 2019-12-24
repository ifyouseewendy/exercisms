use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug, Hash)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> Option<BucketStats> {
    let mut marked : HashSet<(u8, u8)> = HashSet::new();
    let mut queue: VecDeque<(u8, u8, u8)> = VecDeque::new();

    match start_bucket {
        Bucket::One => {
            marked.insert((capacity_1, 0));
            marked.insert((0, capacity_2));
            queue.push_back((capacity_1, 0, 1));
        },
        Bucket::Two => {
            marked.insert((0, capacity_2));
            marked.insert((capacity_1, 0));
            queue.push_back((0, capacity_2, 1));
        }
    };

    while let Some((one, two, moves)) = queue.pop_front() {
        if one == goal { return Some(BucketStats{ moves, goal_bucket: Bucket::One, other_bucket: two}); }
        if two == goal { return Some(BucketStats{ moves, goal_bucket: Bucket::Two, other_bucket: one}); }

        let mut options = vec![];

        // keep one and empty two
        options.push((one, 0));
        // keep one and fill two
        options.push((one, capacity_2));

        // empty one and keep two
        options.push((0, two));
        // fill one and keep two
        options.push((capacity_1, two));

        // pour from one to two
        let next_one = if one + two > capacity_2 { one + two - capacity_2 } else { 0 };
        let next_two = if one + two > capacity_2 { capacity_2 } else { one + two };
        options.push((next_one, next_two));

        // pour from two to one
        let next_one = if one + two > capacity_1 { capacity_1 } else { one + two };
        let next_two = if one + two > capacity_1 { one + two - capacity_1 } else { 0 };
        options.push((next_one, next_two));

        options.iter().for_each(|&(next_one, next_two)| {
            if !marked.contains(&(next_one, next_two)) {
                marked.insert((next_one, next_two));
                queue.push_back((next_one, next_two, moves + 1));
            }
        });
    }

    None
}
