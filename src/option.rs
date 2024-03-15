use std::collections::{hash_map::IntoIter, HashMap};

pub struct OptionIter {
    options: IntoIter<String, String>,
}

impl OptionIter {
    pub fn new(options: HashMap<String, String>) -> Self {
        OptionIter {
            options: options.into_iter(),
        }
    }
}

impl Default for OptionIter {
    fn default() -> Self {
        OptionIter {
            options: HashMap::new().into_iter(),
        }
    }
}

impl Iterator for OptionIter {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.options.next()
    }
}
