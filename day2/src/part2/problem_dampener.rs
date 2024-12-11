fn report_safety(report: Vec<i32>) -> bool {
    let min_diff = 1;
    let max_diff = 3;

    let n = report.len();
    if n < 2 {
        return false
    }

    let mut diff = report[1] - report[0];
    if diff == 0 {
        return false
    }

    if n < 3 {
        return true
    }

    let positive = diff > 0;
    if positive {
        if diff > max_diff || diff < min_diff {
            return false
        }
    } else {
        if diff < -max_diff || diff > -min_diff {
            return false
        }
    }

    for i in 2..n {
        diff = report[i] - report[i-1];
        if diff == 0 {
            return false
        }
        if diff > 0 {
            if diff > max_diff || diff < min_diff {
                return false
            }
            if !positive {
                return false
            }
        }
        if diff < 0 {
            if diff < -max_diff || diff > -min_diff {
                return false
            }
            if positive {
                return false
            }
        }
    }
    true
}

pub fn problem_dampener(report: Vec<i32>) -> bool {
    let mut res = report_safety(report.clone());
    if res {
        return true
    }

    let n = report.len();
    let mut adjusted_report: Vec<i32>;
    for i in 0..n {
        adjusted_report = [&report.clone()[0..i as usize], &report.clone()[i+1 as usize..]].concat();
        res = report_safety(adjusted_report.clone());
        if res {
            return true
        }
    }
    return false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_problem_dampener() {
        let mut a: Vec<i32> = vec![1,2,3,4];
        assert_eq!(true, problem_dampener(a));

        a = vec![1,2,3,4,3];
        assert_eq!(true, problem_dampener(a));

        a = vec![4,3,2,1];
        assert_eq!(true, problem_dampener(a));

        a = vec![4,3,2,1,2];
        assert_eq!(true, problem_dampener(a));

        a = vec![4,4,2,1];
        assert_eq!(false, problem_dampener(a));

        a = vec![7,2,1];
        assert_eq!(false, problem_dampener(a));

        a = vec![1,2,7];
        assert_eq!(true, problem_dampener(a));

        a = vec![1,2,3,7,4];
        assert_eq!(true, problem_dampener(a));

        a = vec![7,6,4,2,1];
        assert_eq!(true, problem_dampener(a));
        a = vec![1,2,7,8,9];
        assert_eq!(false, problem_dampener(a));
        a = vec![9,7,6,2,1];
        assert_eq!(false, problem_dampener(a));
        a = vec![1,3,2,4,5];
        assert_eq!(true, problem_dampener(a));
        a = vec![8,6,4,4,1];
        assert_eq!(true, problem_dampener(a));
        a = vec![1,3,6,7,9];
        assert_eq!(true, problem_dampener(a));

    }
}

