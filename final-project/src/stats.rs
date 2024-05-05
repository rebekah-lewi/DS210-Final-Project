use std::collections::HashMap;

pub fn average(distances: HashMap<i32, i32>) -> f32 {
    let mut total = 0;
    let mut count = 0;

    for data in distances.values() {
        total += data;
        count += 1;
    }
    if count == 0 {
        0.0 
    } else {
        total as f32 / count as f32
    }
}

pub fn max_distance(distances: HashMap<i32, i32>) -> i32 {
    let mut values: Vec<i32> = Vec::new();
    for data in distances.values() {
        values.push(*data);
    }
    values.sort_by(|a, b| b.cmp(a));
    values[0]
}

pub fn find_threshold(distances: HashMap<i32, i32>) -> (usize, usize, usize) {
    let mut less_seven = 0;
    let mut less_fifteen = 0;
    let mut greater_fifteen = 0;

    for value in distances.values() {
        if *value < 7 {
            less_seven += 1;
        } else if *value <= 15 {
            less_fifteen += 1;
        } else {
            greater_fifteen += 1;
        }
    }

    (less_seven, less_fifteen, greater_fifteen)
}