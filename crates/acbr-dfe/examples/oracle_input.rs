use std::io::{self, BufRead};

fn main() {
    for line in io::stdin().lock().lines() {
        let n: i32 = line.expect("leitura").parse().expect("inteiro i32");
        println!("{}", u8::from(acbr_dfe::validar_codigo_uf(n)));
    }
}
