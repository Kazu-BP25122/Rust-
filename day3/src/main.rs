fn main() {
    let a: u8 = 0x12;
    let b: u16 = 0x34;
    //型変換
    let c: u16 = (a as u16) + b;
    println!("{}", c);
    let example = add8(0x0F, 0x01);
    println!("result:{}, carry:{}, half:{}", example.0, example.1, example.2);

    let ram = vec![0x12u8, 0x34, 0x56];
    let f = first_byte(ram);
    println!("{:#04X} {}", f.0, f.1.len());
}

//8bit加算器
fn add8(a: u8, b: u8) ->(u8, bool, bool) {
    //.overflowing_add()は結果と桁あふれがあるかを返す
    let (result, carry) = a.overflowing_add(b);
    let lower_a = a & 0x0F;
    let lower_b = b & 0x0F;
    //比較式はそれ自体がbool型を返す
    let half = (lower_a + lower_b) >= 0x10;
    (result, carry, half)
}

//所有権を返すこともできる
fn first_byte(v: Vec<u8>) -> (u8, Vec<u8>) {
    (v[0], v)//この順を逆にすると、vの所有権が無くんってしまうのでエラーになる
}