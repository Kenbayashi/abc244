use std::usize;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        t: Chars,
    }

    let straight = |((x, y), (d_x, d_y)): ((i64, i64), (i64, i64))| {
        ((x + d_x, y + d_y), (d_x, d_y))
    };

    let right = |((x, y), (d_x, d_y)): ((i64, i64), (i64, i64))| {
        ((x, y), (d_y, d_x * -1))
    };

    let ((x, y), _) = t.into_iter()
                    .fold(((0, 0), (1, 0)), |acc, c| if c == 'S' {straight(acc)} else {right(acc)});

    println!("{} {}", x, y);
}
