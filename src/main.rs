// fn main() {
//     println!("First Program in RUST!");
// }

// Studying Data Types

// Variables can be mutable in one scope but immutable in another. Once a mutable variable goes out of scope, any re-declaration of that variable will need a new mut declaration if mutability is required.

// fn main() {
//     let mut a = 5;
//     {
//         let a = a; // Shadowing, a is now immutable
//                    // a = 6; // Error: cannot assign to `a` because it is immutable
//     }
//     a = 6; // This is allowed because we are back to the outer mutable scope
//     println!("The value of a is: {}", a);
// }

// By default, function parameters are immutable. If you want to change the value of a parameter, you must make it mutable within the function's scope.

// fn main() {
//     let mut z = 10;
//     change_value(&mut z);
//     println!("The value of z after change is: {}", z);
// }

// fn change_value(val: &mut i32) {
//     *val = 20; // Dereferencing and changing the value
// }

// fn main() {
//     print!("Hello World");
//     let sentence = "This is a sentence in string";
//     print!("{} ", sentence);
// }

// fn increment(n: &mut i32) {
//     *n += 2;
// }

// fn main() {
//     let mut v = 14;
//     increment(&mut v);
//     println!("New value after increment {}", v);
// }

// &str is immutable, but String is mutable

// fn main() {
//     let mut string: String = String::from("This is an mutable sentence");
//     println!("{}", string);

//     string.push_str(" Yup Yup");
//     println!("New ,{}", string);
// }

// July 30th . Started Tuple Data Type

// fn main() {
//     let emp_info: (u8, bool, &str) = (12, true, "Lokesh");

//     // destructuring

//     let (age, status, name) = emp_info;

//     println!("Details = {} , {}, {}", age, status, name);
// }

// Write a program to add two numbers

// fn add(a: &mut i32, b: &mut i32) -> i32 {
//     let mut result: i32 = *a + *b;
//     println!("Result before values were modified {}", result);

//     *a += 2;
//     *b += 2;

//     result = *a + *b;
//     println!("Result after values were modified {}", result);

//     return result;
// }

// fn main() {
//     let mut a = 12;
//     let mut b = 18;

//     let _res = add(&mut a, &mut b);
// }

// fn main() {
//     // copy_numbers();
//     // copy_string();
// }

// fn copy_numbers() {
//     let a = 100;
//     let b = a;
//     println!("a is {}", a);
//     println!("b is {}", b);
// }

// fn copy_string() {
//     let s1 = String::from("Hello");
//     let s2 = s1;
//     println!("s1 is {}", s1);
//     println!("s2 is {}", s2);
// }

// In RUST, whenever we use a dynamic memory paradigm, (like in this case, String uses the heap memory unlike the normal integer which uses stack memory), and assign the value of a variable to a different variable, there is a transfer of onwership , as there can only be one owner of a value. Now if like in this example, we try to access s1 after transfer of ownership, it shows an error as s1 is now invalidated.

// Ownership of Functions

// fn main() {
//     let x = String::from("Sparsh");
//     process_integer(x); // Now ownership of x is transferred to function process_integer, and hence x is invalidated now. So if we try the following statement, it will give an error

//     // println!("Value of x in main {}", x);
// }

// fn process_integer(x: String) {
//     println!("Value of x in process_integer {}", x);
// }

// fn main() {
//     let s1: String = get_string();
//     println!("This is s1 {}", s1); // s1 gets the ownership of new_string variable

//     let s2: String = String::from("world");
//     let s3: String = send_get_string(s2); // Now s2 has relinquished the ownership as it is being transferred to s3 over here

//     println!("This is s3 {}", s3);

//     let s4 = get_string();
//     println!("Here is s4 {}",s4);

//     let s4 = get_string();
//     println!("Here is s5 {}",s5);

// }

// fn get_string() -> String {
//     let new_string = String::from("Hello"); // new_string owns "Hello"
//     return new_string; // Ownership of new_string is returned
// }

// fn send_get_string(received_string: String) -> String {
//     return received_string; // Onwership of received_string returned
// }

// fn main() {
//     let s1: String = String::from("hello");
//     let (s2, len) = calculate_length(s1);
//     println!("The string and its length {} {}", s2, len);
// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();
//     (s, length)
// }

// In this example, since ownership will be transferred from s1 to s, we transfer the ownership back by modifying what we return from the function. This is a tedious method

// This is a much better way. Here we pass the reference to the String, which doesnt transfer the ownership, but also allows read-only operations on the said argument. This is the concept of borrowing.

fn main() {
    let s1: String = String::from("Siddharamaiya");
    let mut s2: String = String::from("Drake");
    let length = calculate_length(&s1);
    println!("Here is the length of s1 {}", length);
    let another_length = manipulate_length(&mut s2);
    println!("Here is the length of s2 {} {}", another_length, s1);
}

fn calculate_length(s: &String) -> usize {
    let length = s.len();
    length
}

fn manipulate_length(s: &mut String) -> usize {
    s.push_str("Amother One");
    let length: usize = s.len();
    length
}
