use std::cmp::Ordering;

pub fn post_increment<T: num::Num + Copy>(value: &mut T) -> T {
    let result = *value;
    *value = result + T::one();
    result
}

pub trait IteratorIntoSorted<T> {
    fn into_sorted_by<F: FnMut(&T, &T) -> Ordering>(self, f: F) -> Vec<T>;
}
impl<T, I: Iterator<Item = T>> IteratorIntoSorted<T> for I {
    fn into_sorted_by<F: FnMut(&T, &T) -> Ordering>(self, f: F) -> Vec<T> {
        let mut vec = self.collect::<Vec<_>>();
        vec.sort_by(f);
        vec
    }
}

pub trait VecIntoSorted<T> {
    fn into_sorted_by<F: FnMut(&T, &T) -> Ordering>(self, f: F) -> Vec<T>;
}
impl<T> VecIntoSorted<T> for Vec<T> {
    fn into_sorted_by<F: FnMut(&T, &T) -> Ordering>(mut self, f: F) -> Vec<T> {
        self.sort_by(f);
        self
    }
}
