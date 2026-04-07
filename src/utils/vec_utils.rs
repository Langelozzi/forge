use std::collections::HashSet;

pub fn difference(base: Vec<String>, exclude: Vec<String>) -> Vec<String> {
    let exclude_set: HashSet<String> = exclude.into_iter().collect();
    base.into_iter()
        .filter(|s| !exclude_set.contains(s))
        .collect()
}
