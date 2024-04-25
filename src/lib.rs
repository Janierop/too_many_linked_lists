#![allow(dead_code, unused_variables)]

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod second;
pub mod third;
pub mod fourth;
pub mod fifth;
pub mod sixth;
pub mod sixth_faulty;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
