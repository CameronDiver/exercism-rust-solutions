// fn getStandardStr(n: i64) -> &'static str {
//     return format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.", n, n-1);
// }

pub fn verse(n: i64) -> String {
    match n {
        0 => {
            return String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
        },
        1 => {
            return String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
        },
        2 => {
            return String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
        }
        _ => {
            return format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n-1);
        }
    }
}

pub fn sing(from: i64, to: i64) -> String {
    let mut song = String::new();
    let mut i = from;
    while i >= to {
        song += &verse(i);
        if i != to {
            song += "\n";
        }
        i -= 1;
    }
    println!("{}", song);

    return song;
}
