use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;

/// Returns the string contents of the first matched capture group.
pub fn re_capture_group<'a>(re: &Regex, text: &'a str) -> Option<&'a str> {
    re.captures(text)
        .and_then(|c| c.get(1))
        .and_then(|g| Some(g.as_str()))
}

/// Returns the string contents of the matched capture groups.
pub fn re_capture_groups<'a>(re: &Regex, text: &'a str) -> Option<Vec<&'a str>> {
    re.captures(text).map(|c| {
        c.iter()
            .skip(1)
            .filter_map(|x| x)
            .map(|x| x.as_str())
            .collect_vec()
    })
}

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

pub trait AsSome<'a, T> {
    fn as_some(&'a self) -> &'a T;
    fn as_some_mut(&'a mut self) -> &'a mut T;
}
impl<'a, T> AsSome<'a, T> for Option<T> {
    fn as_some(&'a self) -> &'a T {
        self.as_ref().expect("option should have some value")
    }

    fn as_some_mut(&'a mut self) -> &'a mut T {
        self.as_mut().expect("option should have some value")
    }
}

pub trait Tap<T> {
    fn tap<F: FnOnce(&T) -> ()>(self, f: F) -> T;
}
impl<T, E> Tap<Result<T, E>> for Result<T, E> {
    fn tap<F: FnOnce(&Result<T, E>) -> ()>(self, f: F) -> Result<T, E> {
        f(&self);
        self
    }
}
