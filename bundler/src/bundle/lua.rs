use std::{collections::HashMap, fmt::Display};

pub trait LuaPortable {
    fn into_lua(self) -> String;
}

impl<T> LuaPortable for Vec<T>
where
    T: Display,
{
    fn into_lua(self) -> String {
        let items = self
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<_>>();
        ["{", items.join(",").as_ref(), "}"].join("")
    }
}

impl<T> LuaPortable for HashMap<T, T>
where
    T: Display,
{
    fn into_lua(self) -> String {
        let items = self
            .iter()
            .map(|(k, v)| format!("[\"{}\"]=\"{}\"", k, v))
            .collect::<Vec<_>>();
        ["{", items.join(",").as_ref(), "}"].join("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use big_s::S;
    use rstest::rstest;

    #[rstest]
    #[case::empty(vec![], r#"{}"#)]
    #[case::simgle_item(vec![S("a")], r#"{"a"}"#)]
    #[case::multiple_items(vec![S("a"), S("b")], r#"{"a","b"}"#)]
    fn str_vector(#[case] arg: Vec<String>, #[case] exp: String) {
        let act = arg.into_lua();
        assert_eq!(exp, act);
    }

    #[rstest]
    #[case::empty(HashMap::new(), r#"{}"#)]
    #[case::single_element({
        let mut m: HashMap<String, String> = HashMap::new();
        m.insert(S("a"), S("1"));
        m
    }, r#"{["a"]="1"}"#)]
    #[case::multiple_elements({
        let mut m: HashMap<String, String> = HashMap::new();
        m.insert(S("a"), S("1"));
        m.insert(S("a"), S("2"));
        m
    }, r#"{["a"]="2"}"#)]
    fn str_str_map(#[case] arg: HashMap<String, String>, #[case] exp: String) {
        let act = arg.into_lua();
        assert_eq!(exp, act);
    }
}
