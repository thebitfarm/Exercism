use std::collections::BTreeMap;


pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut ret_map = BTreeMap::<char, i32>::new();

    h.iter().for_each( |(k, v)| {

        v.iter().flat_map(|&c| c.to_lowercase()).for_each(|c| {
            ret_map.entry(c).or_insert(*k);
        });
        
    });

    ret_map
}
