use proconio::input;

fn main() {
    let mut ans = 1i64;
    let mut used = Vec::<i64>::new();

    loop {
        while used.contains(&ans) {
            ans += 1;
        }

        println!("{}", ans);
        used.push(ans);

        input! {
            n: i64,
        }

        if n != 0 {
            used.push(n);
        } else {
            break;
        }
    }
}
