// Calculate the sum of an array which contains integers and other arrays with integers.
// For example: sum([1,2,[3,4,[5]]]) would return 15.
// https://users.rust-lang.org/t/an-idiomatic-way-to-sum-up-values-in-a-multidimensional-array/9485
use std::iter::Sum;

/// Implements sum for any nested set of arrays by flat_mapping them and then calling sum, which
/// either recursively calls this function again (if the types are iterable) or calls the iterator
/// sum type.
pub fn sum<T, U, V>(i: U) -> T
    where T: Sum,
          U: IntoIterator<Item = V>,
          V: IntoIterator<Item = T>
{
    i.into_iter().flat_map(IntoIterator::into_iter).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(sum(vec![vec![1, 2], vec![3, 4, 5, 6]]), 21);
    }
}
