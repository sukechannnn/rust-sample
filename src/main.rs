fn main() {
    // String
    let s1: String = String::from("Hello, World!");
    let s2: &str = &s1;
    println!("{s1}");
    println!("{s2}");

    // tuple
    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";
    println!("{:?}", t);

    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    a[1] = b[1];
    println!("{:?}", &a[0..3]);

    // struct
    struct Person {
        name: String,
        age: u32,
    }
    let person = Person {
        name: String::from("John"),
        age: 8,
    };
    println!("{:?}", format!("{}: {}", "person", person.name));
    println!("{:?}", format!("{}: {}", "age", person.age));

    // Enum
    #[derive(Debug)]
    enum Event {
        Quit,
        KeyDown(u32),
        MouseDown { x: i32, y: i32 },
    }
    let e1 = Event::Quit;
    let e2: Event = Event::KeyDown(3);
    let e3: Event = Event::MouseDown { x: 10, y: 20 };
    println!("{:?}", e1);
    println!("{:?}", e2);
    println!("{:?}", e3);

    fn witch_event(event: Event) -> String {
        match event {
            Event::Quit => "quit".to_string(),
            Event::KeyDown(3) => "3".to_string(),
            Event::KeyDown(x) => x.to_string(),
            Event::MouseDown { x: 10, y: 10 } => "10".to_string(),
            _ => unreachable!(),
        }
    }
    println!("{:?}", witch_event(Event::KeyDown(3)));
    println!("{:?}", witch_event(Event::KeyDown(4)));

    // Option
    fn checked_division(dividend: i32, divisor: i32) -> Option<i32> {
        if divisor == 0 {
            // Failure is represented as the `None` variant
            None
        } else {
            // Result is wrapped in a `Some` variant
            Some(dividend / divisor)
        }
    }
    println!("{}", checked_division(4, 2).unwrap());
    println!("{}", checked_division(4, 0).unwrap_or(-1));

    fn try_division(dividend: i32, divisor: i32) {
        // `Option` values can be pattern matched, just like other enums
        match checked_division(dividend, divisor) {
            None => println!("{} / {} failed!", dividend, divisor),
            Some(quotient) => {
                println!("{} / {} = {}", dividend, divisor, quotient)
            }
        }
    }
    try_division(4, 2);
    try_division(2, 0);

    // Result
    let result: Result<i32, String> = Ok(200);
    println!("code: {}", result.unwrap_or(-1));
    let result: Result<i32, String> = Err("error".to_string());
    println!("code: {}", result.unwrap_or(-1));

    fn print_code(code: i32) -> Result<(), String> {
        println!("print_code: {}", code);
        Ok(())
    }
    let result: Result<i32, String> = Ok(200);
    let _ = result.and_then(print_code);
    let result: Result<i32, String> = Err("error".to_string());
    let _ = result.and_then(print_code);

    fn error_handling(result: Result<i32, String>) -> Result<(), String> {
        let code = result?;
        println!("error_handling: {}", code);
        Ok(())
    }
    let result: Result<i32, String> = Ok(200);
    let _ = error_handling(result);
    let result: Result<i32, String> = Err("ERROR".to_string());
    let error = error_handling(result);
    println!("error_handling error: {}", error.unwrap_err());

    // Vec
    let mut v1 = vec![1, 2, 3, 4, 5];
    for v in v1.clone() {
        println!("{}", v.clone())
    }
    for v in v1.iter_mut() {
        *v = *v + 1;
    }
    println!("{:?}", v1);

    let vec = vec![1, 2, 3];
    if let Some(num) = vec.get(0) {
        println!("vec: {}", num);
    }

    // Box
    fn print(s: Box<[u8]>) {
        println!("{:?}", s)
    }
    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    print(Box::new(byte_array));
    let byte_array = [b'w', b'o', b'r', b'l', b'd', b'!'];
    print(Box::new(byte_array));

    // if
    let number = 1;
    if 0 < number {
        println!("0 < number");
    } else if number < 0 {
        println!("number < 0");
    } else {
        println!("0 == number");
    }

    fn abs(num: i32) -> i32 {
        if num < 0 {
            -num
        } else {
            num
        }
    }
    println!("{}", abs(1));
    println!("{}", abs(-1));

    // loop
    let mut count = 0;
    'main: loop {
        println!("main loop start");
        'sub: loop {
            println!("sub loop start");

            if count == 1 {
                break 'main;
            }
            println!("sub loop end");
            count += 1;
        }
        println!("main loop end")
    }
}
