fn scope_test_1() {
    let some_var = {
        let s = "Hello world";
        s
    };
    println!("Значение переменной some_var: {}", some_var);
    // println!("Значение переменной внутренней области видимости s: {}", s);  // буде ошибка,
    // т.к. s находится в другой (вложенной) области видимости
}

fn string_test_1() {
    let mut s = String::from("Hello");
    println!("До: {}", s);
    s.push_str(", world");
    println!("После: {}", s)
}

fn string_test_2() {
    {
        let mut s = String::from("Hello");
        println!("До: {}", s);
    };
    // s.push_str(", world"); // ошибка not found in this scope
    // println!("После: {}", s)
}

fn copy_string_test_1() {
    let s = String::from("Hello");
    let s2 = s;
    // println!("s: {}", s);  // ошибка: borrow of moved value: `s`
    println!("s2: {}", s2);
}

fn copy_string_test_2() {
    let s = String::from("Hello");
    let s2 = s.clone();
    println!("s: {}", s); // ошибка: borrow of moved value: `s`
    println!("s2: {}", s2);
}

fn main() {
    scope_test_1();
    string_test_1();
    string_test_2();
    copy_string_test_1();
    copy_string_test_2();
}
