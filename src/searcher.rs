use super::village::Village;
use rayon::prelude::*;

pub struct Searcher {
    villages: Vec<Village>,
}

impl Searcher {
    pub fn new(villages: Vec<Village>) -> Self {
        Self {
            villages
        }
    }

    pub fn by_name(&self, name: String) -> Vec<Village> {
        let result: Vec<Village> = self.villages.
            iter().filter_map(| v | 
                if v.name() == name { Some(v.clone()) } 
                else { None }).collect();
        result
    }

    pub fn element_count(&self) -> usize {
        return self.villages.len();
    }
}
