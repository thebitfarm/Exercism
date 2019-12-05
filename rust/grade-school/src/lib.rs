use std::collections::HashMap;

#[derive(Debug)]
pub struct School {
    roster: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let e = self.roster.entry(grade).or_insert( vec![] );
        e.push(student.to_string());
        e.sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        //let mut v = vec![];
        //self.roster.keys().into_iter().for_each(|&k| { v.push(k); });
        //v.sort();
        //v
        self.roster.keys().map(|k| k.clone()).collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.roster.get(&grade) {
            //Some(val) => { let mut v = val.clone(); v.sort(); Some(v) },
            Some(val) => Some(val.clone()),
            None => None
        }
    }
}
