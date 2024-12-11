pub fn report_safety(report: Vec<i32>) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_safety() {
        let mut a: Vec<i32> = vec![1,2,3,4];
        assert_eq!(true, report_safety(a));

        a = vec![1,2,3,4,3];
        assert_eq!(false, report_safety(a));

        a = vec![4,3,2,1];
        assert_eq!(true, report_safety(a));

        a = vec![4,3,2,1,2];
        assert_eq!(false, report_safety(a));

        a = vec![4,4,2,1];
        assert_eq!(false, report_safety(a));

        a = vec![7,2,1];
        assert_eq!(false, report_safety(a));

        a = vec![1,2,7];
        assert_eq!(false, report_safety(a));
    }
}

