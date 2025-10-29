pub fn get_average(list: &[i32]) -> f64 {
    // let mut sum_ = 0;
    // for &el in list.iter() {
    //     sum_ += el;
    // }

    let sum_: i32 = list.iter().sum();

    let result = sum_ as f64 / list.len() as f64;

    println!("{:?}", list);

    result
}
