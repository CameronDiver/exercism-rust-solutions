pub fn raindrops(input: i64) -> String {
    let mut ret = String::new();
    let mut worded = false;
    if input % 3 == 0 {
        ret.push_str("Pling");
        worded = true;
    }
    if input % 5 == 0 {
        ret.push_str("Plang");
        worded = true;
    }
    if input % 7 == 0 {
        ret.push_str("Plong");
        worded = true;
    }
    if !worded {
        return input.to_string();
    } else {
        return ret;
    }
}
