fn main() {
    println!("Hello, world!");

    let x = 3;
    println!("x: {}", x);

    let mut y = 3;
    println!("original y: {}", y);
    y = 5;
    println!("changed y: {}", y);

    let z = 3;
    let z = z + 2;
    let z = z * 2;
    println!("z: {}", z);

    let z = "Hello, I am a string";
    println!("Now z: {}", z);

    const MAX_NUM: u32 = (1 << 31) - 1;
    println!("MAX_NUM: {}", MAX_NUM);

    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    for i in 1..10 {
        println!("i = {}", i);
    }

    let sum: i32 = (1..=100).sum();
    println!("sum of 1..100 is {}", sum);

    let array: [i32; 20] = [1; 20];
    println!("Length of array is {}", array.len());

    struct Student {
        name: &'static str,
        score: u32,
    }

    let score = 99;
    let name = "Zachary Yan";
    let mut zachary = Student {
        name,
        score
    };

    zachary.score = 100;
    println!("name: {}, score: {}", zachary.name, zachary.score);
}
