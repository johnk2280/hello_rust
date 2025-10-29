pub mod chapter_8;

use chapter_8::task_1;

pub fn main() {
    let list = vec![1, 2, 5, 6, 10];

    println!("Среднее значение = {}", task_1::get_average(&list));
}
