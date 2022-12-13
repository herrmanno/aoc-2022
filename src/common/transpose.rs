//! Contains traits and structures for definint transposition on two-dimentional structures
//!
//! # Example
//!
//! ```rust
//! # use aoc2022::common::transpose::Transpose;
//! let v = vec![vec![1,2,3], vec![4,5,6]];
//! let out: Vec<Vec<i32>> = v.into_iter().transpose().collect_all();
//! assert_eq!(out, vec![vec![1,4], vec![2,5], vec![3,6]])
//! ```

/// Describe two-dimensional structures that can be transposed
pub trait Transpose: IntoIterator
where
    Self: IntoIterator + Sized,
    Self::Item: IntoIterator,
{
    fn transpose(self) -> Transposed<Self>;
}

/// The transposition of a two-dimensional structure
impl<T> Transpose for T
where
    T: IntoIterator + Sized,
    <T as IntoIterator>::Item: IntoIterator,
{
    fn transpose(self) -> Transposed<Self> {
        Transposed {
            iters: self.into_iter().map(|inner| inner.into_iter()).collect(),
        }
    }
}

/// The transposition of a two-dimensional structure
///
/// # Example
/// ```rust
/// # use aoc2022::common::transpose::Transpose;
/// let v = vec![vec![1,2], vec![3,4]];
/// let mut transposed = v.into_iter().transpose();
/// assert_eq!(transposed.next().unwrap().collect::<Vec<u8>>(), vec![1,3]);
/// assert_eq!(transposed.next().unwrap().collect::<Vec<u8>>(), vec![2,4]);
/// ```
pub struct Transposed<I>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    iters: Vec<<I::Item as IntoIterator>::IntoIter>,
}

impl<I> Transposed<I>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    /// Collects inner and outer iterator(s)
    #[allow(dead_code)]
    pub fn collect_all<T, U>(self) -> T
    where
        T: FromIterator<U>,
        U: FromIterator<<I::Item as IntoIterator>::Item>,
    {
        self.map(|inner| inner.collect()).collect()
    }

    /// Collect inner iterators
    #[allow(dead_code)]
    pub fn collect_inner<T>(self) -> impl Iterator<Item = T>
    where
        T: FromIterator<<I::Item as IntoIterator>::Item>,
    {
        self.map(|inner| inner.collect())
    }

    /// Collect inner iterators w/ collector function
    pub fn collect_with<T, U, F>(self, inner_collector: F) -> T
    where
        T: FromIterator<U>,
        F: FnMut(std::vec::IntoIter<<<I as IntoIterator>::Item as IntoIterator>::Item>) -> U,
    {
        self.map(inner_collector).collect()
    }
}

impl<I> Iterator for Transposed<I>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    type Item = std::vec::IntoIter<<I::Item as IntoIterator>::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_items = vec![];
        for it in self.iters.iter_mut() {
            match it.next() {
                Some(el) => {
                    next_items.push(el);
                }
                None => {
                    return None;
                }
            }
        }
        Some(next_items.into_iter())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vec_transpose() {
        let v1 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let out1: Vec<Vec<i32>> = v1
            .into_iter()
            .transpose()
            .map(|inner| inner.collect())
            .collect();
        assert_eq!(out1, vec![vec![1, 4], vec![2, 5], vec![3, 6]])
    }

    #[test]
    fn test_vec_transpose_collect_all() {
        let v1 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let out1: Vec<Vec<i32>> = v1.into_iter().transpose().collect_all();
        assert_eq!(out1, vec![vec![1, 4], vec![2, 5], vec![3, 6]])
    }

    #[test]
    fn test_vecdeque_transpose_collect_all() {
        use std::collections::VecDeque;

        let v1 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let out1: VecDeque<VecDeque<i32>> = v1.into_iter().transpose().collect_all();
        let expected = VecDeque::from([
            VecDeque::from([1, 4]),
            VecDeque::from([2, 5]),
            VecDeque::from([3, 6]),
        ]);
        assert_eq!(out1, expected)
    }

    #[test]
    fn test_option_transpose_collect_inner() {
        let v1 = vec![Some(1), Some(2)];
        let out1: Option<Vec<i32>> = v1.into_iter().transpose().collect_inner().next();
        assert_eq!(out1, Some(vec![1, 2]))
    }

    #[test]
    fn test_option_transpose_collect_inner_early_exit() {
        let v1 = vec![Some(1), None, Some(2)];
        let out1: Option<Vec<i32>> = v1.into_iter().transpose().collect_inner().next();
        assert_eq!(out1, None)
    }

    #[test]
    fn test_result_transpose_collect_inner() {
        let v1: Vec<Result<i32, ()>> = vec![Ok(1), Ok(2)];
        let out1: Result<Vec<i32>, ()> =
            v1.into_iter().transpose().collect_inner().next().ok_or(());
        assert_eq!(out1, Ok(vec![1, 2]))
    }

    #[test]
    fn test_collect_with() {
        let v1 = vec![vec![Some(1), None, Some(2)], vec![Some(3), Some(4), None]];
        let out1: Vec<Vec<i32>> = v1
            .into_iter()
            .transpose()
            .collect_with(|inner| inner.flatten().collect());
        assert_eq!(out1, vec![vec![1, 3], vec![4], vec![2]])
    }
}
