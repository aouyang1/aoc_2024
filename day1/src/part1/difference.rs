pub fn difference(mut a_id: Vec<i32>, mut b_id: Vec<i32>) -> i32 {
    assert_eq!(a_id.len(), b_id.len());

    a_id.sort();
    b_id.sort();

    let n = a_id.len();

    let mut total = 0;
    for i in 0..n {
        let diff: i32 = a_id[i] - b_id[i];
        total = total + diff.abs();
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference() {
        let a: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
        let b: Vec<i32> = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(11, difference(a, b))
    }
}
