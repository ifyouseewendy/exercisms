use std::cmp::Ordering;
use std::collections::BTreeMap;

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
    let mut table = BTreeMap::new();

    match_results.lines().for_each(|line| {
        let parts: Vec<&str> = line.split(';').collect();

        table.entry(parts[0]).or_insert_with(|| TeamRecord::new(parts[0]));
        table.entry(parts[1]).or_insert_with(|| TeamRecord::new(parts[1]));

        match parts[2] {
            "win" => {
                table.entry(parts[0]).and_modify(|tr| tr.win());
                table.entry(parts[1]).and_modify(|tr| tr.loss());
            },
            "draw" => {
                table.entry(parts[0]).and_modify(|tr| tr.draw());
                table.entry(parts[1]).and_modify(|tr| tr.draw());
            },
            "loss" => {
                table.entry(parts[0]).and_modify(|tr| tr.loss());
                table.entry(parts[1]).and_modify(|tr| tr.win());
            },
            _ => (),
        }
    });

    let mut records: Vec<TeamRecord> = table.values().cloned().collect();
    records.sort();
    println!("{records:#?}", records=records);

    let mut ret = vec![];
    ret.push(format!("{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", "Team", "MP", "W", "D", "L", "P"));

    records.iter().rev().for_each(|tr|
        ret.push(format!("{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", tr.name, tr.mp, tr.w, tr.d, tr.l, tr.p))
    );

    ret.join("\n")
}
