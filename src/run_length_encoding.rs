#[allow(dead_code)]
fn run_length_encoding<T: std::cmp::PartialEq + Copy>(x: Vec<T>) -> Vec<(T, i64)> {
    let mut ret: Vec<(T, i64)> = Vec::new();

    let l: usize = x.len();

    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < l {
        while j < l && x[i] == x[j] {
            j += 1usize;
        }

        ret.push((x[i], (j-i) as i64));
        i = j;
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_char() {
        let s: &str = "AAABBBBBAACDD";
        let v: Vec<char> = s.chars().collect();
        let actual: Vec<(char, i64)> = run_length_encoding(v);
        let expected: Vec<(char, i64)> = vec![('A', 3), ('B', 5), ('A', 2), ('C', 1), ('D', 2)];
        assert_eq!(actual, expected);
    }
}