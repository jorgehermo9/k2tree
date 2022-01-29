
mod matrix;
mod k2tree;
mod sequence;
mod webgraph;

pub use crate::k2tree::K2tree;
pub use crate::matrix::Matrix;
pub use crate::sequence::Sequence;
pub use crate::webgraph::*;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
