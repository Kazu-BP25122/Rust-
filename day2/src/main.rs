fn main() {
    let mut a: Vec<i32> = Vec::new();
    a.push(2);
    a.push(4);
    println!("a[1] = {}", a[1]);
    const DATE: i32 = 1219;
    println!("MyBirthday = {}", DATE);
    let c = Explorer {
        name: String::from("Alan"),
        str: 8,
    };

    println!("name:{}\nSTR:{}", c.name, c.str);
}

struct Explorer {
    name: String,
    str: i32,
}