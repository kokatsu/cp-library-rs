#[allow(dead_code)]
fn compress<T: Clone + Ord + PartialEq>(x: &[T]) -> Vec<usize> {
    let mut ranking: Vec<T> = x.to_owned();
    ranking.sort();
    ranking.dedup();

    x.iter().map(|u| ranking.partition_point(|p| p < u)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    // https://atcoder.jp/contests/tessoku-book/tasks/tessoku_book_o
    #[test]
    fn test1() {
        let a: Vec<i64> = vec![46, 80, 11, 77, 46];
        let actual: Vec<usize> = compress(&a).iter().map(|&f| f + 1).collect();
        let expected: Vec<usize> = vec![2, 4, 1, 3, 2];
        assert_eq!(actual, expected);
    }

    // https://atcoder.jp/contests/abc036/tasks/abc036_c
    #[test]
    fn test2() {
        let a: Vec<i64> = vec![3, 3, 1, 6, 1];
        let actual: Vec<usize> = compress(&a);
        let expected: Vec<usize> = vec![1, 1, 0, 2, 0];
        assert_eq!(actual, expected);
    }
}