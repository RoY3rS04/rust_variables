fn main() {

    //Shadowing
    let x = 5;

    let x = x + 1;

    let x = x*2;

    println!("The value of x is: {}", x);

    //Integer Types

    //Floating-Point Types
    let x = 2.0;

    let y: f32 = 3.0;

    //Numeric Operations

    let sum = 5+10;

    let difference = 95.5 - 4.3;

    let product = 4*30;

    let quotient = 56.7 / 32.2;

    let remainder = 43 % 5;

    //Boolean Type
    let t = true;

    let f: bool = false;

    //Character Type
    let c = 'z';

    let z = ' ';

    let heart_eyed_cat = ' ';

    //Compound Types

    // ==> Tuples <== //
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //Tuple Destructuring
    let (x,y,z) = tup;

    //Accessing Tuple Values By The Tuple index
    let five_hundred = tup.0;

    let six_point_four = tup.1;

    let one = tup.2;

    // ==> Arrays <== //
    //In Rust arrays have a fixed length

    let a = [1,2,3,4,5];

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];

    //Accessing to array elements

    let first = a[0];
    let second = a[1];
    let last = a[a.len() - 1];

    //Trying to access to an index that is greater than the array lenght will cause an error

    let index = 10;

    let element = a[index];

    println!("The value of element is: {}", element);

}
