use std::any::type_name;

#[cfg(debug_assertions)]
#[allow(dead_code)]
pub fn type_of<T>(_: &T) -> String {
    String::from(type_name::<T>())
}
