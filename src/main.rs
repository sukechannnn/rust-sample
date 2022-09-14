fn main() {
    let s1: String = String::from("Hello, World!");
    let s2: &str = &s1;
    println!("{s1}");
    println!("{s2}");

    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";
    println!("{:?}", t);

    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    a[1] = b[1];
    println!("{:?}", &a[0..3]);
}

