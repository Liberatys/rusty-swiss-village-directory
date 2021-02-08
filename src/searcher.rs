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
        println!("{:?}", self.villages);
        let village_result: Vec<Village> = self.villages.clone().into_par_iter().filter(|village| village.name == name).collect();
        village_result
    }

    pub fn element_count(&self) -> usize {
        return self.villages.len();
    }
}
