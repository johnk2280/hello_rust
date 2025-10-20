// fn scope_test_1() {
//     let some_var = {
//         let s = "Hello world";
//         s
//     };
//     println!("Значение переменной some_var: {}", some_var);
//     // println!("Значение переменной внутренней области видимости s: {}", s);  // буде ошибка,
//     // т.к. s находится в другой (вложенной) области видимости
// }
//
// fn string_test_1() {
//     let mut s = String::from("Hello");
//     println!("До: {}", s);
//     s.push_str(", world");
//     println!("После: {}", s)
// }
//
// fn string_test_2() {
//     {
//         let mut s = String::from("Hello");
//         println!("До: {}", s);
//     };
//     // s.push_str(", world"); // ошибка not found in this scope
//     // println!("После: {}", s)
// }
//
// fn copy_string_test_1() {
//     let s = String::from("Hello");
//     let s2 = s;
//     // println!("s: {}", s);  // ошибка: borrow of moved value: `s`
//     println!("s2: {}", s2);
// }
//
// fn copy_string_test_2() {
//     let s = String::from("Hello");
//     let s2 = s.clone();
//     println!("s: {}", s); // ошибка: borrow of moved value: `s`
//     println!("s2: {}", s2);
// }

// fn func_scopes_test_1() {
//     let s = String::from("Hello");
//     takew_ownership(s);
//
//     let x = 5;
//     make_copy(x);
//     println!("x: {}", x);
// }
//
// fn takew_ownership(some_string: String) {
//     println!("{}", some_string);
// }
//
// fn make_copy(some_integer: i32) {
//     println!("{}", some_integer);
// }
//

// fn gives_ownership() -> String {
//     let some_string = String::from("Hello");
//     some_string
// }
//
// fn takes_and_gives_back(some_str: String) -> String {
//     some_str
// }
//
// fn func_scopes_test_2() {
//     let s1 = gives_ownership();
//     let s2 = String::from("Hello");
//     let s3 = takes_and_gives_back(s2);
// }

// fn calculate_len(some_str: &String) -> usize {
//     some_str.len()
// }
//
// fn returning_params() {
//     let s1 = String::from("Hello");
//     let len = calculate_len(&s1);
//     println!("Длина строки `{}` равна {}", s1, len);
// }

// fn calucalete_area_tuple() {
//     let rect_1 = (30, 50);
//     println!("Площадь прямоугольника равна {} кв. пикселей", area(rect_1));
// }
//
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn calucalete_area_struct() {
//     let rect_1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     println!(
//         "Площадь прямоугольника {:#?} равна {} кв. пикселей",
//         rect_1,
//         area(&rect_1),
//     );
// }
//
// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
//
//     fn can_hold(&self, other: &Rectangle) -> bool {
//         self.width > other.width && self.height > other.height
//     }
// }
//
// fn calucalete_area_struct_with_method() {
//     let rect_1 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//     let rect_2 = Rectangle {
//         width: 10,
//         height: 40,
//     };
//     let rect_3 = Rectangle {
//         width: 60,
//         height: 70,
//     };
//
//     println!(
//         "Площадь прямоугольника {:?} равна {} кв. пикселей",
//         rect_1,
//         rect_1.area(),
//     );
//
//     println!(
//         "Может ли прямоугольник {:?} содержать в себе прямоугольник {:?}? Ответ: {}",
//         rect_1,
//         rect_2,
//         rect_1.can_hold(&rect_2)
//     );
//     println!(
//         "Может ли прямоугольник {:?} содержать в себе прямоугольник {:?}? Ответ: {}",
//         rect_1,
//         rect_3,
//         rect_1.can_hold(&rect_3)
//     );
// }

// #[derive(Debug)]
// enum IpAddrKind {
//     V4,
//     V6,
// }

// #[derive(Debug)]
// struct IpAddr {
//     version: IpAddrKind,
//     address: String,
// }

// fn enum_test_1() -> (IpAddrKind, IpAddrKind) {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;
//     (four, six)
// }

// fn enum_test_2() {
//     let localhost = IpAddr {
//         version: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };
//
//     println!("{:?}", localhost);
// }

// #[derive(Debug)]
// enum IpAddr {
//     V4(String),
//     V6(String),
// }
//
// fn enum_test_3() {
//     let localhost = IpAddr::V4(String::from("127.0.0.1"));
//     println!("{:?}", localhost);
// }
//

// fn test_option_1() {
//     let some_number = Some(5);
//     let some_string = Some("Строковый литерал");
//     let absent_number: Option<i32> = None;
//     let concrete_number = 4;
//
//     println!("{}", some_number + concrete_number);
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn pattern_matching_test_1(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    // scope_test_1();
    // string_test_1();
    // string_test_2();
    // copy_string_test_1();
    // copy_string_test_2();
    // func_scopes_test_1();
    // func_scopes_test_2();
    // returning_params();
    // calucalete_area_struct_with_method();

    // println!(
    //     "Значения перечисления {:?} {:?}",
    //     enum_test_1().0,
    //     enum_test_1().1
    // )

    // enum_test_2();
    // enum_test_3();

    // test_option_1();
    let coin_ = pattern_matching_test_1(Coin::Quarter);
    println!("{}", coin_);
}
