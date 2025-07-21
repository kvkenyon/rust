pub fn largest<T: PartialOrd>(v: &[T]) -> Result<&T, &str> {
    if v.len() == 0 {
        return Err("Empty array.");
    }
    let mut max = &v[0];
    for i in 1..v.len() {
        if &v[i] > &max {
            max = &v[i];
        }
    }
    Ok(max)
}
