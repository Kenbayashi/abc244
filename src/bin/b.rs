use std::usize;

use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        t: Chars,
    }

    let straight = |((x, y), (d_x, d_y))| ((x + d_x, y + d_y), (d_x, d_y));

    let right = |(pos, (d_x, d_y))| (pos, (d_y, d_x * -1));

    let ((x, y), _) = t.into_iter()
                       .fold(((0i64, 0i64), (1i64, 0i64)), |acc, c| if c == 'S' {straight(acc)} else {right(acc)});

    println!("{} {}", x, y);
}
