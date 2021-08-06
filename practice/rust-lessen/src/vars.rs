pub mod sub_a;
pub mod sub_b;

const MAX: u32 = 100_000;

pub fn run() {
    println!("vars module");

    // variables are default immutable
    // can mutable by adding "mut" in front of variable name
    let mut x = 5; // <- mutable
    println!("x is {}.", x);
    x = 6; // can bind 6
    println!("x is {}.", x);
    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS);
    println!("memory address of MAX is : {:p}", &MAX);

    let i2: i64 = 1;
    let i3: i64 = 2;
    println!("Stack address of i2 is : {:p}", &i2);
    println!("Stack address of i3 is : {:p}", &i3);

    // shadowing
    let y = 5;
    println!("Stack address of i3 is : {:p}", &y);
    let y = y + 1;
    println!("Stack address of i3 is : {:p}", &y);
    let y = y * 2;
    println!("Stack address of i3 is : {:p}", &y);
    println!("y is {}", y);
    {
        let y = 0;
        println!("y is {}", y);
    }
    println!("y is {}", y);

    // taple
    let t1 = (500, 6.4, "dummy");
    let (x, y, z) = t1;
    println!("x, y, z value is {} {} {}", x, y, z);
    println!("t1 value is {} {} {}", t1.0, t1.1, t1.2);

    let mut t2 = ((0, 1), (2, 3));
    println!("t2 value is {:?}", t2);
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t2;
    *x1_ptr = 5;
    *y1_ptr = -5;
    println!("t2 value is {:?}", t2);

    // array
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[1], a2[3]);

    // slice
    let s1 = "helloこんにちは"; // 20bytes
    let s2 = "helle"; // 5bytes
    println!("Stack address of s1 is : {:p}", &s1);
    println!("Stack address of s2 is : {:p}", &s2);
    println!("Static memory address of s1 is : {:?}", s1.as_ptr());
    println!("Static memory address of s2 is : {:?}", s2.as_ptr());
    println!("s1 lengsh is {}", s1.len());
    println!("s2 lengsh is {}", s2.len());

    // string
    let mut str1 = String::from("hello");
    let mut str2 = String::from("helloworld");
    println!("Stack address of str1 is : {:p}", &str1);
    println!("Stack address of str2 is : {:p}", &str2);
    println!("Heap memory address of s1 is : {:?}", str1.as_ptr());
    println!("Heap memory address of s2 is : {:?}", str2.as_ptr());
    println!("str1 lengsh is {}", str1.len());
    println!("str2 lengsh is {}", str2.len());
    println!("str1 capacity is {}", str1.capacity());
    println!("str2 capacity is {}", str2.capacity());
    str1.push_str("_new");
    str2.push_str("_new");
    println!("{} {}", str1, str2);
}
