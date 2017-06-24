fn square(x: i64) -> i64 {
    x * x
}

pub fn square_of_sum(n: i64) -> i64 {
    square((0..n+1).sum())
}

pub fn sum_of_squares(n: i64) -> i64 {
    (0..n+1).map(square).sum()
}

pub fn difference(n: i64) -> i64 {
    square_of_sum(n) - sum_of_squares(n)
}
