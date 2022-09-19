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
}
