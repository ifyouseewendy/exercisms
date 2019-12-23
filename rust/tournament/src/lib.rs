use std::fmt;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq)]
struct TeamRecord {
    name: String,
    mp: u8,
    w: u8,
    d: u8,
    l: u8,
    p: u8,
}
impl Ord for TeamRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.p == other.p {
            other.name.cmp(&self.name)
        } else {
            self.p.cmp(&other.p)
        }
    }
}

impl PartialOrd for TeamRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TeamRecord {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Default for TeamRecord {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            mp: 0,
            w: 0,
            d: 0,
            l: 0,
            p: 0,
        }
    }
}

impl fmt::Display for TeamRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            self.name, self.mp, self.w, self.d, self.l, self.p,
        )
    }
}

impl TeamRecord {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn win(&mut self) {
        self.mp += 1;
        self.w += 1;
        self.p += 3;
    }
    pub fn draw(&mut self) {
        self.mp += 1;
        self.d += 1;
        self.p += 1;
    }
    pub fn loss(&mut self) {
        self.mp += 1;
        self.l += 1;
    }
}

pub fn tally(match_results: &str) -> String {
    let mut map = HashMap::new();

    match_results.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(';').collect();
        let home   = parts[0];
        let away   = parts[1];
        let result = parts[2];

        map.entry(home).or_insert_with(|| TeamRecord::new(home));
        map.entry(away).or_insert_with(|| TeamRecord::new(away));

        match result {
            "win" => {
                map.entry(home).and_modify(|tr| tr.win());
                map.entry(away).and_modify(|tr| tr.loss());
            },
            "draw" => {
                map.entry(home).and_modify(|tr| tr.draw());
                map.entry(away).and_modify(|tr| tr.draw());
            },
            "loss" => {
                map.entry(home).and_modify(|tr| tr.loss());
                map.entry(away).and_modify(|tr| tr.win());
            },
            _ => (),
        }
    });


    let mut records: Vec<TeamRecord> = map.values().cloned().collect();
    records.sort();

    let header = format!("{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", "Team", "MP", "W", "D", "L", "P");
    let mut table = vec![header];

    table.extend(
        records.iter().rev().map(|tr| tr.to_string()).collect::<Vec<String>>()
    );

    table.join("\n")
}
