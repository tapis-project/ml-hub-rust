pub fn is_vec_empty<T>(v: &Option<Vec<T>>) -> bool {
    match v {
        Some(vec) => vec.is_empty(),
        None => true, // also skip if None
    }
}