pub fn average(distances: Vec<&str>) -> f32 {
    let mut total = 0;
    let mut count = 0;

    for data in distances {
        let parts: Vec<&str> = data.split(':').collect();
        if let Some(value_str) = parts.get(1) {
            if let Ok(num) = value_str.trim().parse::<i32>() {
                total += num;
                count += 1;
            } else {
                println!("Warning: Failed to parse '{}'", value_str);
            }
        }
    }
    if count == 0 {
        0.0 
    } else {
        total as f32 / count as f32
    }
}

pub fn max_distance(distanes: Vec<&str>) -> Option<i32> {
    let mut max_value: Option<i32> = None;

    for data in distances {
        let parts: Vec<&str> = data.split(':').collect();
        if let Some(value_str) = parts.get(1) {
            if let Ok(num) = value_str.trim().parse::<i32>() {
                max_value = match max_value {
                    Some(current_max) => Some(current_max.max(num)),
                    None => Some(num),
                };
            } else {
                println!("Warning: Failed to parse '{}'", value_str);
            }
        }
    }
    max_value
}

pub fn find_threshold(distances: Vec<&str>) -> (usize, usize, usize) {
    let mut less_seven = 0;
    let mut less_fifteen = 0;
    let mut greater_fifteen = 0;

    for data in distances {
        let parts: Vec<&str> = data.split(':').collect();
        if let Some(value_str) = parts.get(1) {
            if let Ok(num) = value_str.trim().parse::<i32>() {
                if num < 7 {
                    less_seven += 1;
                } else if num <= 15 {
                    less_fifteen += 1;
                } else {
                    greater_fifteen += 1;
                }
            } else {
                println!("Warning: Failed to parse '{}'", value_str);
            }
        }
    }

    (less_seven, less_fifteen, greater_fifteen)
}
