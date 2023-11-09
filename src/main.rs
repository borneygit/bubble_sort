use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt::Debug;

#[warn(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    #[warn(dead_code)]
    fn new(name: String, age: u32) -> Self {
        Self { name, age }
    }
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.age.partial_cmp(&other.age)
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.age == other.age
    }
}

fn main() {}

fn array_bubble_sort<T: PartialOrd>(vec: &mut Vec<T>) {
    for i in (1..vec.len()).rev() {
        for j in 0..i {
            if &vec[j] > &vec[j + 1] {
                vec.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::array_bubble_sort;
    use crate::Person;

    #[test]
    fn test_i32_array() {
        let mut arr = vec![3, 2, 5, 4, -1, 0, 7];
        array_bubble_sort(&mut arr);
        assert_eq!(&arr, &[-1, 0, 2, 3, 4, 5, 7]);
    }

    #[test]
    fn test_partialold_array() {
        let mut persons = vec![
            Person::new(String::from("alice"), 24),
            Person::new(String::from("bob"), 32),
            Person::new(String::from("dave"), 21),
            Person::new(String::from("borney"), 33),
        ];
        array_bubble_sort(&mut persons);

        assert_eq!(
            &persons,
            &[
                Person::new(String::from("dave"), 21),
                Person::new(String::from("alice"), 24),
                Person::new(String::from("bob"), 32),
                Person::new(String::from("borney"), 33),
            ]
        );
    }
}
