pub fn build_proverb(list: &[&str]) -> String {

    let mut ret = String::new();


    for (i, word) in list.iter().enumerate() {
        //println!(" idx[{}] , word: {}", i, word);

        if i < list.len()-1 {
            ret.push_str(&format!("For want of a {} the {} was lost.\n", word, list[i+1]));
        }
        
    }

    if list.len() > 0 {
        ret += &format!("And all for the want of a {}.", list[0]);
    }

    ret
}
