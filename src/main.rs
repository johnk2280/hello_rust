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

fn calucalete_area_tuple() {
    let rect_1 = (30, 50);
    println!("Площадь прямоугольника равна {} кв. пикселей", area(rect_1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
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
    calucalete_area_tuple();
}
