#[macro_use]
extern crate rutie;
extern crate csv;

extern crate time;
use time::PreciseTime;

#[macro_use]
extern crate lazy_static;

class!(SwissVillageDirectory);
class!(Village);

wrappable_struct!(village::Village, VillageWrapper, VILLAGE_WRAPPER);
use rutie::{Object, Class, RString, AnyObject, VM};

mod village;
pub mod loader;
pub mod searcher;

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
  SwissVillageDirectory,
  _itself,

  fn pub_find_by_name(name: RString) -> AnyObject {
    let name_str = name.
          map_err(|e| VM::raise_ex(e)).
          unwrap().to_string();
    let searched = SEARCHER.by_name(name_str);
    Class::from_existing("Village").wrap_data(searched[0].clone(), &*VILLAGE_WRAPPER)
  }
);

methods!(
  Village,
  itself,

  fn village_name() -> RString {
    RString::new_utf8(&itself.get_data(&*VILLAGE_WRAPPER).name())
  }

  fn village_inspect() -> RString {
    RString::new_utf8(&format!("{}", itself.get_data(&*VILLAGE_WRAPPER).name()))
  }
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
        assert_eq!(result[0].name(), "Aeugstertal");
    }

    // #[test]
    fn load_test() {
        for x in 0..1000 {
            let result = SEARCHER.by_name("Aeugstertal".to_string());
            assert_eq!(result[0].name(), "Aeugstertal");
        }
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn Init_rusty_swiss_village_directory() {
  Class::new("SwissVillageDirectory", None).define(|itself| {
    itself.def_self("by_name", pub_find_by_name);
  });

  Class::new("Village", None).define(|itself| {
    itself.def("name", village_name);
  });
}
