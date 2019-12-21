use std::collections::BTreeMap;
use std::collections::BinaryHeap;

#[derive(Default)]
pub struct School(BTreeMap<u32, BinaryHeap<String>>);

impl School {
    pub fn new() -> School {
        Default::default()
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.0.entry(grade)
            .and_modify(|s| s.push(student.to_string()))
            .or_insert_with(|| BinaryHeap::from(vec![student.to_string()]));
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().copied().collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.0.get(&grade).map(|bh| bh.clone().into_sorted_vec())
    }
}
