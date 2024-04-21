fn c1() {
    let strings = vec!["1", "abc", "2", "def", "3"];


    //     过滤并尝试将字符串转换为整数
    let numbers: Vec<i32> = strings
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    // let numbers = strings
    //     .iter()
    //     // 过滤并尝试将字符串转换为整数
    //     .filter_map(|s| s.parse().ok());

    println!("Numbers: {:?}", numbers);
    println!("strings: {:?}", strings);
}

fn c2() {
    let strings = vec!["1", "2", "3", "4", "5"];

    let even_numbers: Vec<&str> = strings
        .iter()
        .filter(|&s| {
            if let Ok(n) = s.parse::<i32>() {
                return n % 2 == 0;
            }
            false
        })
        .copied()
        .collect();

    println!("Even numbers: {:?}", even_numbers);
    println!("strings: {:?}", strings);
}

fn main() {
  // c1();
  c2();
}
