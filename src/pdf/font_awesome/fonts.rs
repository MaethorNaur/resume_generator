use std::collections::HashMap;

lazy_static! {
    pub static ref FONTS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("cake", "");
        m.insert("envelope", "");
        m.insert("gitlab", "");
        m.insert("github", "");
        m.insert("linkedin", "");
        m.insert("twitter", "");
        m.insert("phone", "");
        m.insert("map-marker", "");
        m
    };
}
