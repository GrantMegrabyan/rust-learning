fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let target = 3;
    
    let result = binary_search(&v, target);
    println!("{} - {:?}", target, result);
}

fn binary_search(v: &Vec<i32>, target: i32) -> Option<&i32> {
    let mut lo = 0;
    let mut hi = v.len() - 1;

    if target < v[0] {
        return None
    }

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if v[mid] == target {
            return Some(&v[mid]);
        }

        if v[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn existing_items() {
        let v = vec![1, 3, 5, 7, 9, 10, 13, 15, 16, 18, 24, 26, 28];
        let targets = v.clone();

        for target in targets {
            assert_eq!(Some(&target), binary_search(&v, target))
        }
    }

    #[test]
    fn non_existing_items() {
        let v = vec![1, 3, 5, 7, 9, 10, 13, 15, 16, 18, 24, 26, 28];
        let targets = vec![0, 2, 4, 11, 17, 27, 30];

        for target in targets {
            assert_eq!(None, binary_search(&v, target))
        }
    }
}