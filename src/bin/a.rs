fn main() {
    proconio::input! {
        n: usize,
        s: proconio::marker::Chars,
    };

    println!("{}", s[n - 1]);
}
