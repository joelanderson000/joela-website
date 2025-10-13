

use yew::{classes, Properties};

#[derive(Properties, PartialEq)]
pub struct Style {
    utilities: String,
}

impl Style {
    fn from(string: &str) -> Self {
        Self {
            utilities: String::from(string.trim()),
        }
    }

    fn to_string(&self) -> String {
        self.utilities.clone()
    }
}


macro_rules! tailwind {
    ($utilities:literal) => {
        {
            // let mut utility_list = Vec::new();
            // utility_list.push($utilities.split_whitespace().collect());
            let utility_list: Vec<&str> = $utilities.split_whitespace().collect();
            // $(
            //     utility_list.push($utility);
            // )*
            classes!(utility_list)
        }
    }
}

macro_rules! combined_classes {
    ($utilities:literal) => {
        {
            classes!(String::from($utilities))
        }
    }
}
pub(crate) use combined_classes;
pub(crate) use tailwind;
