pub fn sort<T: PartialOrd + Copy>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }

    let middle = data.len() / 2;

    sort(&mut data[..middle]);
    sort(&mut data[middle..]);

    merge(data, middle);
}

fn merge<T: PartialOrd + Copy>(data: &mut [T], middle: usize) {
    let left = data[..middle].to_vec();
    let right = data[middle..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            data[k] = left[i];
            i += 1;
        } else {
            data[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        data[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        data[k] = right[j];
        j += 1;
        k += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regular() {
        let mut data = [3, 5, 2, 1, 4, 6, 9, 0, 8, 7];
        sort(&mut data);
        assert_eq!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], data);
    }

    #[test]
    fn test_reversed() {
        let mut data = [9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        sort(&mut data);
        assert_eq!([0, 1, 2, 3, 4, 5, 6, 7, 8, 9], data);
    }

    #[test]
    fn test_duplicates() {
        let mut data = [4, 2, 1, 0, 3, 3, 2, 4, 1, 0];
        sort(&mut data);
        assert_eq!([0, 0, 1, 1, 2, 2, 3, 3, 4, 4], data);
    }
}
