fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;
    let mut x = 5;
    let x = x + 2;
    let dec = 98_222;
    let hex = 0xff;
    let oct = 0o77;
    let bin: usize = 0b1111_0000;
    let byt = b'A';

    let flo1 = 2.0;
    let flo2: f32 = 3.0;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'Z';

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.1;

    let arr = [1,2,3,4,5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let arr2 = [3; 5];

    let first = arr2[0];
    
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
