use std::fs::File;
use std::io::{Error as IoError, Read};
use std::path::PathBuf;

fn main() {
    /*
    panic!("Farewell");

    let v = vec![0, 1, 2, 3];
    println!("{}", v[6]);


    let fruits = vec!["banana", "apple","coconut", "orange", "strawberry"];
    let first = fruits.get(0);
    println!("{:?}", first);
    let third = fruits.get(2);
    println!("{:?}", third);
    let non_existant = fruits.get(99);
    println!("{:?}", non_existant);

    for &index in [0, 2, 99].iter() {
        match fruits.get(index) {
            Some(&"coconut") => println!("Coconuts are"),
            Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
            None => println!("There's no fruit!")
        }
    }

    match some_number {
        Some(7) => println!("That's my lucky number"),
        _ => {}
    }    

    let some_number: Option<u8> = Some(7);

    if let Some(7) = some_number {
        println!("That's my lucky number");
    }


    let gift = Some("candy");
    assert_eq!(gift.unwrap(), "candy");
    let empty_gift: Option<&str> = None;
    assert_eq!(empty_gift.unwrap(), "candy")

    let a = Some("value");
    assert_eq!(a.expect("fruits are healthy"), "value");
    let b: Option<&str> = None;
    b.expect("fruits are healthy");

    assert_eq!(Some("dog").unwrap_or("cat"), "dog");
    assert_eq!(None.unwrap_or("cat"), "cat");
    let john = Person {
        first: String::from("James"),
        middle: Some(String::from("Olivier")),
        last: String::from("Smith")
    };

    assert_eq!(build_full_name(&john), "James Olivier Smith");

    let alice = Person {
        first: String::from("Alice"),
        middle: None,
        last: String::from("Stevens")
    };

    assert_eq!(build_full_name(&alice), "Alice Stevens");

    let bob = Person {
        first: String::from("Robert"),
        middle: Some(String::from("Murdock")),
        last: String::from("Jones")
    };

    assert_eq!(build_full_name(&bob), "Robert Murdock Jones");
  
    println!("{:?}", safe_division(0.0, 3.0));
    println!("{:?}", safe_division(4.0, 0.0));
    println!("{:?}", safe_division(0.0,2.0));
  */

  assert!(read_file_contents(PathBuf::from("src/main.rs")).is_ok());
  assert!(read_file_contents(PathBuf::from("yolo.txt")).is_err());







}

fn read_file_contents(path: PathBuf) -> Result<String, IoError> {
    let mut string = String::new();

    let mut file: File = match File::open(path) {
        Ok(file_handle) => file_handle,
        Err(io_error) => return Err(io_error),
    };

    match file.read_to_string(&mut string) {
        Ok(_) => (),
        Err(io_error) => return Err(io_error)
    };

    Ok(string)
}
/*

#[derive(Debug)]
struct DivisionByZeroError;

fn safe_division(dividend: f64, divisor: f64) -> Result<f64, DivisionByZeroError> {
    if divisor == 0.0 {
        Err(DivisionByZeroError)
    } else {
        Ok(dividend / divisor)
    }
}

struct Person {
    first: String,
    middle: Option<String>,
    last: String
}

fn build_full_name(person: &Person) -> String{
    let mut full_name = String::new();
    full_name.push_str(&person.first);
    full_name.push_str(" ");

    match &person.middle {
        Some(string) => {
            full_name.push_str(string);
            full_name.push_str(" ");
        },
        _ => {}
    }

    full_name.push_str(&person.last);
    full_name
}
*/