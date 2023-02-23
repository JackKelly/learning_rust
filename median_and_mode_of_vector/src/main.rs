fn main() {
    let list = vec![5, 2, 7, 1, 0, 0, 1, 1];
    println!("Middle value = {}", get_median(&list));
    println!("Mode value = {}", get_mode(&list));
}


fn get_median(v: &Vec<i32>) -> i32 {
    let mut v = Vec::from_iter(v);
    v.sort();
    let i_middle = v.len() / 2;
    *v[i_middle]
}


fn get_mode(v: &Vec<i32>) -> i32 {
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    let mut max_count = 0;
    let mut mode = 0;

    for i in v {
        let count = counts.entry(*i).or_insert(0);
        *count += 1;

        if *count > max_count {
            max_count = *count;
            mode = *i;
        }
    }

    mode
}