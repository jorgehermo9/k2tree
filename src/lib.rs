
pub mod matrix;
pub mod k2tree;
pub mod sequence;

pub use k2tree::K2tree;
pub use matrix::Matrix;
pub use sequence::Sequence;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
