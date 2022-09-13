fn main() {
    let s1: String = String::from("Hello, World!");
    let s2: &str = &s1;
    println!("{s1}");
    println!("{s2}");

    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";
    println!("{:?}", t);
}

