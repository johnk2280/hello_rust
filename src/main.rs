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

fn main() {
    scope_test_1();
    string_test_1();
}
