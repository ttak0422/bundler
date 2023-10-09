use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn to_unique_vector<T: Hash + Eq>(v: Vec<T>) -> Vec<T> {
    v.into_iter().collect::<HashSet<_>>().into_iter().collect()
}

pub fn to_unique_map<T: Hash + Eq>(m: HashMap<&str, Vec<T>>) -> HashMap<&str, Vec<T>> {
    m.into_iter()
        .map(|(k, v)| (k, to_unique_vector(v)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest(arg, exp,
        case(vec![], vec![]),
        case(vec!["a"], vec!["a"]),
        case(vec!["a", "b", "c", "a"], vec!["a", "b", "c"]),
    )]
    fn make_unique_vector(arg: Vec<&str>, exp: Vec<&str>) {
        let act = to_unique_vector(arg);

        assert_eq!(
            itertools::sorted(exp).collect::<Vec<_>>(),
            itertools::sorted(act).collect::<Vec<_>>()
        );
    }

    #[rstest(arg, exp,
        case(HashMap::<&str, Vec<&str>>::from([]), HashMap::<&str, Vec<&str>>::from([])),
        case(HashMap::<&str, Vec<&str>>::from([("a", vec!["1"])]), HashMap::<&str, Vec<&str>>::from([("a", vec!["1"])])),
        case(HashMap::<&str, Vec<&str>>::from([("a", vec!["1","2","3","1"])]), HashMap::<&str, Vec<&str>>::from([("a", vec!["1","2","3"])])),
    )]
    fn make_unique_map(arg: HashMap<&str, Vec<&str>>, exp: HashMap<&str, Vec<&str>>) {
        let act = to_unique_map(arg);

        assert_eq!(exp.keys().len(), act.keys().len());
        assert_eq!(
            itertools::sorted(exp.keys()).collect::<Vec<_>>(),
            itertools::sorted(act.keys()).collect::<Vec<_>>()
        );
        assert_eq!(
            exp.values()
                .map(|value| itertools::sorted(value).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
            act.values()
                .map(|value| itertools::sorted(value).collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        );
    }
}
