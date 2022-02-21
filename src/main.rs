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

    // Unicode support
    let heart_eyed_cat = '😻';
    println!("heart_eyed_cat: {}", heart_eyed_cat);

    // How to use range
    for i in 1..10 {    // Open range
        print!("i = {}\t", i);
    }
    println!();

    let sum: i32 = (1..=100).sum(); // Close range
    println!("sum of 1..100 is {}", sum);

    // How to use tuple
    let _tup1: (i8, f32, bool) = (-10, 2.718281828459, true);
    let tup2 = (20.2, ("Ruby", false));

    let (_, (ruby, _male)) = tup2;   // unwrap a tuple
    println!("name: {}", ruby);

    // How to use array
    let array: [i32; 20] = [1; 20];
    println!("Length of array is {}", array.len());

    // How to use struct
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

    let tristan = Student {
        name: "Tristan Yan",
        ..zachary   // unwrap a struct for rest of fields.
    };

    println!("name: {}, score: {}", tristan.name, tristan.score);

    // How to use enum
    enum FamilyRole {
        Dad,
        Mom,
        Son,
        Daughter
    }

    struct FamilyMember {
        name: &'static str,
        role: FamilyRole,
    }

    let member1 = FamilyMember {
        name: "Zachary",
        role: FamilyRole::Son
    };

    match member1.role
    {
        FamilyRole::Dad => println!("{} is son in family", member1.name),
        FamilyRole::Mom => println!("{} is son in family", member1.name),
        FamilyRole::Daughter => println!("{} is son in family", member1.name),
        FamilyRole::Son => println!("{} is son in family", member1.name)
    }
}
