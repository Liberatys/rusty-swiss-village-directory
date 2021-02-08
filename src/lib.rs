#[macro_use]
extern crate rutie;

extern crate csv;

#[macro_use]
extern crate lazy_static;

module!(RustSwissVillageDirectory);

mod village;
mod loader;
mod searcher;

use rutie::{Module, RString, Boolean, AnyObject, VM};

lazy_static! {
    static ref SEARCHER: searcher::Searcher = {
        let loader = loader::Loader::new("./data/PLZO_CSV_WGS84.csv".to_string());
        let villages = loader.load();
        match villages {
            Ok(villages) => {
                searcher::Searcher::new(villages)
            },
            Err(e) => {
                println!("{:?}", e);
                searcher::Searcher::new(Vec::new())
            }
        }
    };
}

methods!(
  RustSwissVillageDirectory,
  _itself,
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_file() {
        assert_eq!(SEARCHER.element_count() > 1, true);
    }

    #[test]
    fn find_by_name() {
        let result = SEARCHER.by_name("Aeugstertal".to_string());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "Aeugstertal");
        assert_eq!(result[0].zip_code, 8914);
        assert_eq!(result[0].one_digit_spare, 2);
        assert_eq!(result[0].longitude, "8.494");
        assert_eq!(result[0].latitude, "47.283");
    }
}
