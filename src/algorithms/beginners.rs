pub fn avg(slice: &[f64]) -> f64 {
    let mut count: f64 = 0.0;
    for num in slice {
        count += num;
    }
    let len: f64 = slice.len() as f64;
    let result: f64 = count / len;
    result
}

//this function accepts a string and count, and return a new string multiple by count
pub fn repeat_str(src: &str, count: usize) -> String {
    let mut result: String = String::with_capacity(count * src.len());
    for _ in 0..count {
        result.push_str(src);
    }

    return result;
}
