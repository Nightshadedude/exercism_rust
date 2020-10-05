use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut trans = BTreeMap::new();
    for (i,v) in h.iter() {
        for ch in v {
            let c = *ch;
            trans.insert(c.to_ascii_lowercase(), *i);
        }
    }
    trans
}
