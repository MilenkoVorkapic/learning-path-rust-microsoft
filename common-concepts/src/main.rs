// use std::collections::HashMap;

fn main() {
    /*
    let a_number = 10;
    let a_boolean = true;
    let a_number = a_number + 20;
    println!("the number is {}.", a_number);
    println!("the boolean is {}.", a_boolean);
    

    let number:u32 = "42".parse().expect("Not a number!");

    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);
    println!("9 / 2 = {}", 9u32 / 2);
    println!("9 / 2 = {}", 9.0 / 2.0);
    println!("3 * 6 = {}", 3 * 6);
    let is_bigger = 1 > 4;
    println!("{}", is_bigger);

    let c = 'z';
    let z = 'Z';
    let emoji = '😍';

    println!("{} {} {}", c, z, emoji);
    

    let mut helloo = String::from("Hello, ");
    helloo.push('w');
    helloo.push_str("orld!");
    println!("{}", helloo);
    

    let tuple = (
        "hello",
        5,
        'c'
    );
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
    

    struct Person {
        name: String,
        age: u8,
        likes_oranges: bool
    }

    struct Point2D(u32, u32);

    struct Unit;

    let person = Person {
        name: String::from("Adam"),
        likes_oranges: true,
        age: 25
    };

    let origin = Point2D(0, 0);

    let unit = Unit;

    

    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(KeyPress),
        Paste(String),
        Click(Click)
    }

    struct Click {
        x: i64,
        y: i64
    }

    struct KeyPress(char);

    struct Car {
        color: String,
        transmission: Transmission,
        convertible: bool,
        mileage: u32,
    }

    #[derive(PartialEq, Debug)]
    enum Transmission {
        Manual,
        SemiAuto,
        Automatic
    }

    fn car_factory(color: String, transmission: Transmission, convertible: bool) -> Car {
        let car: Car = Car {
            color,
            transmission,
            convertible,
            mileage: 0
        };

        assert_eq!(car.mileage, 0);
        return car;
    }

    let client_request_1 = car_factory(String::from("Red"), Transmission::Manual, false);
    assert_eq!(client_request_1.color, "Red");
    assert_eq!(client_request_1.transmission, Transmission::Manual);
    assert_eq!(client_request_1.convertible, false);

    let client_request_2 = car_factory(String::from("Silver"), Transmission::Automatic, true);
    assert_eq!(client_request_2.color, "Silver");
    assert_eq!(client_request_2.transmission, Transmission::Automatic);
    assert_eq!(client_request_2.convertible, true);

    let client_request_3 = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
    assert_eq!(client_request_3.color, "Yellow");
    assert_eq!(client_request_3.transmission, Transmission::SemiAuto);
    assert_eq!(client_request_3.convertible, false);

    println!("Hello world");
    another_function();
   

    assert_eq!(is_divisible_by(2, 3), false);
    assert_eq!(is_divisible_by(5, 1), true);
    assert_eq!(is_divisible_by(24, 6), true);
    

    let weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    let byte_buffer = [0_u8; 512];
   
    let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    println!("first element of the array: {}.", letters[0]);
    println!("second element of the array: {}.", letters[1]);


    let three_numbers = vec![1, 2, 3];
    println!("Initial vector: {:?}", three_numbers);
    let ten_zeroes = vec![0; 10];
    println!("Ten zeroes: {:?}", ten_zeroes);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("{:?}", v);

    let mut v = vec![1, 2];
    let two = v.pop();

    let mut v = vec![1, 2, 3];
    let three = v[2];
    v[1] = v[1] + 5;
    println!("{:?}", v);

    let v = vec![1, 2, 3, 4, 5];
    let does_not_exist = v[100];
    


    let mut book_reviews: HashMap<String, String> = HashMap::new();
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book".to_string(),
    );
    book_reviews.insert(
        "Grimms' Fairy Tales".to_string(),
        "Masterpiece".to_string(),
    );
    book_reviews.insert(
        "Pride and Prejudice".to_string(),
        "Very enjoyable".to_string(),
    );
    book_reviews.insert(
        "The adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot".to_string(),
    );

    if !book_reviews.contains_key("Les Miserables") {
        println!("We've got {} reviews, but Les Miserables ain't one", book_reviews.len());
    }

    println!("Review for Jane {}", book_reviews["Pride and Prejudice"]);
    // println!("Review for Herman {}", book_reviews["Moby Dick"]);

    let sherlock = "The adventures of Sherlock Holmes";
    assert_eq!(book_reviews.contains_key(sherlock), true);
    book_reviews.remove(sherlock);
    assert_eq!(book_reviews.contains_key(sherlock), false);


    indexing_tuple();
    indexing_array();


 let basket = fruit_basket();
 assert!(basket.len() >= 3, "Basket must have at least three type of fruits");
 assert!(basket.values().sum::<u32>() >= 5, " Basket must have at least 5 fruits");


    if 1 == 2 {
        println!("whoops, math broke");
    } else {
        println!("everything's fine");
    }


    let formal = true;
    let greeting = if formal {
        "Good evening"
    } else {
        "Hello, friend!"
    };
    println!("{}", greeting);


    let number = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }
 

    loop {
        println!("I loop forever");
    }


    let mut i = 1;
    let something = loop {
        i *= 2;
        if i > 100 {
            break i;
        }
    };
    assert_eq!(something, 128);
  

    let mut counter = 0;
    while counter < 10 {
        println!("hello");
        counter = counter +1
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for item in 0..5 {
        println!("{}", item *2);
    }
  */




}

/*
fn fruit_basket() -> HashMap<String, u32>{
    let mut basket: HashMap<String, u32> = HashMap::new();

    basket.insert("Banana".to_string(), 3);
    basket.insert("Tomato".to_string(), 2);
    basket.insert("Blueberries".to_string(), 5);

    basket
}


fn indexing_tuple(){
    let numbers = (1, 2, 3);
    let second = numbers.1;

    assert_eq!(2, second, "This is not the 2nd number in the tuple: {}", second);
}

fn indexing_array(){
    let characters = ['a', 'b', 'c', 'd', 'e'];
    let letter_d = characters[3];

    assert_eq!('d', letter_d, "This is not the character for the letter d: {}", letter_d);
}


fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }
    dividend % divisor == 0
}

fn another_function(){
    println!("Hello from another function!");
}
*/











