mod greeting {

    pub fn greet_world() {
        println!("Hello, World!");
    }

    pub fn greet_by_name(name: String) {
        println!("Hello, {}!", name);
    }

    pub fn greet_by_full_name(fname: &str, lname: &str) {
        let full_name = full_name(fname, lname);
        greet_by_name(full_name);
    }

    // private functions in module
    fn full_name(first_name: &str, last_name: &str) -> String {
        format!("{} {}", first_name, last_name)
    }
}

mod rust02 {
    pub fn looping(limit: i8) {
        println!("Iterating via loop until: {}", limit);
        let mut n = 1;
        loop {
            println!("   with loop: {:?}", n);
            if n == limit {
                break;
            }
            n += 1;
        }
        println!("Iterating via while until: {}", limit);
        let mut n = 1;
        while n < limit {
            println!("   with while: {:?}", n);
            n += 1;
        }
    }
}
mod rust03 {
    pub fn option_match() {
        let value: Option<u8> = Some(10);
        match value {
            Some(_) => println!("option has value in it"),
            None => println!("never will get here"),
        }
    }

    pub fn option_as_variable(o: Option<&str>) {
        match o {
            Some(s) => println!("option value passed as params is: {}", s),
            None => println!("option has no value"),
        }
    }
}

pub fn test_intro() {
    greeting::greet_world();
    greeting::greet_by_name("John".to_string());
    greeting::greet_by_full_name("John", "Doe");

    rust02::looping(4);
    rust03::option_match();
    rust03::option_as_variable(Some("TestingOption"));
    rust03::option_as_variable(None);
}
