fn main() {
    /*
     {
         let mascot = String::from("ferris");
     }
     println!("{}", mascot);


     let mascot = String::from("ferris");
     let _ferris = mascot;
     println!("{}", mascot);


     let greeting = String::from("hello");
     let greeting_ref = &greeting;
     println!("Greeting: {}", greeting);

     let greeting = String::from("Hello");
     print_greeting(&greeting);
     print_greeting(&greeting);
    let mut greeting = String::from("Hello");
     change(&mut greeting);


     let mut value = String::from("value");
     let ref1 = &value;
     let ref2 = &mut value;
     println!("{}, {}", ref1, ref2);
     let x;
     {
         let y = 42;
         x = &y;
     }
     println!("x: {}", x);


    let magic1 = String::from(" hello");
    let result;
    {
        let magic2 = String::from("world");
        result = longest_word(&magic1, &magic2);

    }
    println!("The longest word is: {}", result);


    #[derive(Debug)]
    struct Highlight<'document>(&'document str);

    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{:?}", fox);
    println!("{:?}", dog)
    */

    let name1 = "Joe";
    let name2 = "Chris";
    let name3 = "Anne";

    let mut names = Vec::new();

    assert_eq!("Joe", copy_and_return(&mut names, &name1));
    assert_eq!("Chris", copy_and_return(&mut names, &name2));
    assert_eq!("Anne", copy_and_return(&mut names, &name3));

    assert_eq!(
        names,
        vec!["Joe".to_string(), "Chris".to_string(), "Anne".to_string()]
    )
}

fn copy_and_return<'a>(vector: &'a mut Vec<String>, value: &'a str) -> &'a String {
    vector.push(String::from(value));
    vector.get(vector.len() -1).unwrap()
}

/*
fn longest_word<'lifetime>(x: &'lifetime String, y: &'lifetime String) -> &'lifetime String {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}



fn change(message: &mut String) {
    message.push_str("!");
}


fn print_greeting(message: &String){
    println!("Greeting: {}", message)
}




fn process(input: String){}

fn caller(){
    let s = String::from("Hello");
    process(s);
    process(s);
}


fn process(input: u32){}
fn caller(){
    let n = 1u32;
    process(n);
    process(n);
}

fn process(input: String){}
fn caller(){
    let n = String::from("hello");
    process(n.clone());
    process(n);
}
*/
