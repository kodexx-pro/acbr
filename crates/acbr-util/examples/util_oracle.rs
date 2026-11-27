use std::io::{self, BufRead};
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let (kind, value) = line.split_once('|').unwrap();
        let out = match kind {
            "num" => acbr_util::only_number(value),
            "alpha" => acbr_util::only_alpha(value),
            "alnum" => acbr_util::only_alpha_num(value),
            _ => panic!("tipo"),
        };
        println!("{out}");
    }
}
