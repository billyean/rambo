// #[derive(Debug, Clone)]
// struct DataChunk {
//     value: i32,
// }
//
// fn move_or_copy(dc : &mut DataChunk) -> DataChunk {
//     dc.value += 1;
//     dc.clone()
// }
//
// fn main() {
//     let mut a = DataChunk{ value: 20};
//     let b = move_or_copy(&mut a);
//     let c = move_or_copy(&mut a);
//     println!("{:?}", a);
//     println!("{:?}", b);
//     println!("{:?}", c);
// }


use sqlparser::ast::{Expr, FunctionArg, FunctionArgExpr, SelectItem};
use sqlparser::parser::ParserOptions;

fn visit_where_expr(expr: &Expr, level: usize) {
    match expr {
        Expr::BinaryOp {left, op, right} => {
            println!("{} Operation {}", "  ".repeat(level), op);
            visit_where_expr(left, level + 1);
            visit_where_expr(right, level + 1);
        }
        Expr::Function (function) => {
            println!("{} Call Function {}", "  ".repeat(level), function.name);
            visit_func_args(function.clone().args, level + 1);
        }
        Expr::Identifier(id) => println!("{} Access id {}", "  ".repeat(level), id),
        Expr::Value(value) => println!("{} Access const {}", "  ".repeat(level), value),
        _ => println!("{} Expr {}", "  ".repeat(level), expr)
    }
}

fn visit_func_args(args: Vec<FunctionArg>, level: usize) {
    for arg in args {
        match arg {
            FunctionArg::Unnamed(arg_expr) => match arg_expr {
                FunctionArgExpr::Expr(expr) => visit_where_expr(&expr, level + 1),
                FunctionArgExpr::QualifiedWildcard(on) => println!("{} Access object {}", "  ".repeat(level), on),
                FunctionArgExpr::Wildcard => println!("{} Access wildcard {}", "  ".repeat(level), "*"),
            }
            FunctionArg::Named { name, arg }  =>  match arg {
                FunctionArgExpr::Expr(expr) => visit_where_expr(&expr, level + 1),
                FunctionArgExpr::QualifiedWildcard(on) => println!("{} Access object {}", "  ".repeat(level), on),
                FunctionArgExpr::Wildcard => println!("{} Access wildcard {}", "  ".repeat(level), "*"),
            }
        }

    }
}

fn visit_projection_items(items: Vec<SelectItem>) {
    for item in items {
        println!("Col {}", item);
    }
}

fn main() {
    use sqlparser::dialect::GenericDialect;
    use sqlparser::parser::Parser;

    let sql_where = "select * from t where myfunc(b) = 'a' and d = 5";
    let mut parser_where = Parser::new(&GenericDialect);
    parser_where = parser_where.with_options(ParserOptions {
        trailing_commas: true,
        ..Default::default()
    });
    let ast_where = parser_where.try_with_sql(sql_where).unwrap().parse_select().unwrap().selection.unwrap();
    println!("Print where expr on sql {}", sql_where);
    println!("{}", "-".repeat(80));
    visit_where_expr(&ast_where, 0);


    let sql_projections = "a + b, c, d from t";
    let mut parser_projection = Parser::new(&GenericDialect);
    parser_projection = parser_projection.with_options(ParserOptions {
        trailing_commas: true,
        ..Default::default()
    });
    let ast_projection = parser_projection.try_with_sql(sql_projections).unwrap().parse_select().unwrap().projection;
    println!("Print projection expr on sql {}", sql_projections);
    println!("{}", "-".repeat(80));
    visit_projection_items(ast_projection);
}

// use std::env;
// use std::borrow::Cow;
// use std::thread;
// use std::time::Duration;
// use std::collections::HashMap;
//
//
// struct data_dict {
//     data: i64,
//     validity: bool
// }
//
//
// struct ValidIndex {
//     indexes: Option<Vec<u64>>
// }
//
// // fn get_valid_data(dict: Vec<data_dict>, indexes: Vec<u64>) -> Vec<i64> {
// //     // // Use dictionary establish a map for get valid data offset
// //     // let mut data_map: HashMap<u64, Vec<u64>> = HashMap::new();
// //     // for (i, data) in dict.iter().enumerate() {
// //     //     if data.validity {
// //     //         data_map.insert(i as u64, Vec::new());
// //     //     }
// //     // }
// //
// //     let mut valid_vec = dict.iter().map(|dv| if dv.validity { Some(Vec::new()) } else { None}).collect();
// //     let mut counter = 0;
// //     for index in indexes {
// //         match valid_vec.get(index){
// //             Some(vector) => {
// //                 vector.add(counter);
// //                 counter += 1;
// //             }
// //             None => ()
// //         }
// //     }
// //     // fill the data
// //     let r = Vec::with_capacity(counter);
// //     valid_vec.iter().enumerate().filter()
// // }
//
//     // // Calculate the valid data offset
//     // let mut counter = 0;
//     // for index in indexes {
//     //     match data_map.get(&index){
//     //         Some(vector) => {
//     //             vector.add(counter);
//     //             counter += 1;
//     //         }
//     //         None => ()
//     //     }
//     // }
//     // // fill the data
//     // let r = Vec::with_capacity(counter);
//     // for (i, pos) in data_map.iter() {
//     //     r[pos] = dict[i];
//     // }
//     // r
//
// fn test(a: i32, b: i32) -> i32 {
//     a + b
// }
// fn main() {
//     let a = 1;
//     println!("{:?} +  {:?} = {:?}", a, a, test(a, a));
//     // let v = vec!(1, 1, 2);
//     // let mut x = u64::MAX;
//     // let mut vec = Vec::new();
//     // while x >= 128 {
//     //     vec.push((x as u8) & 0xff);
//     //     x >>= 7;
//     // }
//     // vec.push(x as u8);
//     // println!("{:?}", vec);
//     // println!("{:?}", u64::MAX.to_be_bytes());
//
//     // let v = vec!(1, 1, 2);
//     // // Create an iterator over windows of size 2
//     // for two in v.windows(2) {
//     //     if !two.get(0).eq(&two.get(1)) {
//     //         println!("{} is not same as {}", two.get(0).unwrap(), two.get(1).unwrap(), );
//     //     }
//     // };
//
//
//     // let mut table_names = Vec::new();
//     // for index in 0..10_000_000 {
//     //     table_names.push(format!("{}_{}", "test", index))
//     // }
//     //
//     // let args: Vec<String> = env::args().collect();
//     // let catalog = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
//     // let schema = "yyyyyyyyyyyyyyyyyyyyyyyyyyyyyy";
//     // match args.get(1)   {
//     //
//     //     Some(arg) => {
//     //         match arg.as_str()  {
//     //             "str" =>  {
//     //                 let mut tables: Vec<StrTable> = vec!();
//     //                 for name in &table_names {
//     //                     let table = StrTable {
//     //                         catalog,
//     //                         schema,
//     //                         name,
//     //                     };
//     //                     tables.push(table);
//     //                 }
//     //                 println!("After push with string slice........");
//     //                 thread::sleep(Duration::from_secs(60));
//     //                 println!("{:?}", tables.len());
//     //             }
//     //             _ => {
//     //                 let mut tables: Vec<CowTable> = vec!();
//     //                 let catalog = Cow::from(catalog);
//     //                 let schema = Cow::from(schema);
//     //                 for name in &table_names {
//     //                     let table = CowTable {
//     //                         catalog: catalog.clone(),
//     //                         schema: schema.clone(),
//     //                         name: Cow::from(name),
//     //                     };
//     //                     tables.push(table);
//     //                 }
//     //                 println!("After push with cow of str........");
//     //                 thread::sleep(Duration::from_secs(60));
//     //                 println!("{:?}", tables.len());
//     //             }
//     //         }
//     //
//     //     }
//     //     None => {
//     //         panic!("Expected a input");
//     //     }
//     // }
// }
//
//
// // test.lt = "Not hello world";
// //
// // let mut test1 = Test1 {
// //     lt: Cow::Borrowed("Hello world"),
// // };
// //
// // test1.lt = Cow::Borrowed("Not hello world");
// //
// // println!("{:?}", test);
// // println!("{:?}", test1);
//
// // println!("Hello, world!");
// //
// // let x = 3;
// // println!("x: {}", x);
// //
// // let mut y = 3;
// // println!("original y: {}", y);
// // y = 5;
// // println!("changed y: {}", y);
// //
// // let z = 3;
// // let z = z + 2;
// // let z = z * 2;
// // println!("z: {}", z);
// //
// // let z = "Hello, I am a string";
// // println!("Now z: {}", z);
// //
// // const MAX_NUM: u32 = (1 << 31) - 1;
// // println!("MAX_NUM: {}", MAX_NUM);
// //
// // // Unicode support
// // let heart_eyed_cat = '😻';
// // println!("heart_eyed_cat: {}", heart_eyed_cat);
// //
// // // How to use range
// // for i in 1..10 {    // Open range
// //     print!("i = {}\t", i);
// // }
// // println!();
// //
// // let sum: i32 = (1..=100).sum(); // Close range
// // println!("sum of 1..100 is {}", sum);
// //
// // // How to use tuple
// // let _tup1: (i8, f32, bool) = (-10, 2.718281828459, true);
// // let tup2 = (20.2, ("Ruby", false));
// //
// // let (_, (ruby, _male)) = tup2;   // unwrap a tuple
// // println!("name: {}", ruby);
// //
// // // How to use array
// // let array: [i32; 20] = [1; 20];
// // println!("Length of array is {}", array.len());
// //
// // // How to use struct
// // struct Student {
// //     name: &'static str,
// //     score: u32,
// // }
// //
// // let score = 99;
// // let name = "Zachary Yan";
// // let mut zachary = Student {
// //     name,
// //     score
// // };
// //
// // zachary.score = 100;
// // println!("name: {}, score: {}", zachary.name, zachary.score);
// //
// // let tristan = Student {
// //     name: "Tristan Yan",
// //     ..zachary   // unwrap a struct for rest of fields.
// // };
// //
// // println!("name: {}, score: {}", tristan.name, tristan.score);
// //
// // // How to use enum
// // enum FamilyRole {
// //     Dad,
// //     Mom,
// //     Son,
// //     Daughter
// // }
// //
// // struct FamilyMember {
// //     name: &'static str,
// //     role: FamilyRole,
// // }
// //
// // let member1 = FamilyMember {
// //     name: "Zachary",
// //     role: FamilyRole::Son
// // };
// //
// // match member1.role
// // {
// //     FamilyRole::Dad => println!("{} is son in family", member1.name),
// //     FamilyRole::Mom => println!("{} is son in family", member1.name),
// //     FamilyRole::Daughter => println!("{} is son in family", member1.name),
// //     FamilyRole::Son => println!("{} is son in family", member1.name)
// // }
