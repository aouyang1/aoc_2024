use std::collections::HashMap;

pub fn similarity(a_id: Vec<i32>, b_id: Vec<i32>) -> i32 {
    assert_eq!(a_id.len(), b_id.len());

    let n = a_id.len();

    let mut b_map: HashMap<i32, i32> = HashMap::new();
    for i in 0..n {
        match b_map.get(&b_id[i]) {
            Some(num) => {
                b_map.insert(b_id[i], num+1);
            },
            None => {
                b_map.insert(b_id[i], 1);
            }
        }
    }

    let mut score: i32 = 0;
    for i in 0..n {
        match b_map.get(&a_id[i]) {
            Some(num) => {
                score = score + a_id[i] * num;
            },
            None => (),
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity() {
        let a: Vec<i32> = vec![3, 4, 2, 1, 3, 3];
        let b: Vec<i32> = vec![4, 3, 5, 3, 9, 3];

        assert_eq!(31, similarity(a, b))
    }
}

