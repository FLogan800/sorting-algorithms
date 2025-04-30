pub fn sort<T: PartialOrd + Copy>(data: &mut [T]) {
    for i in 0..data.len() {
        let mut insert_index = i;
        let current_value = data[i];

        for j in (0..i).rev() {
            if data[j] > current_value {
                data[j + 1] = data[j];
                insert_index = j;
            } else {
                break;
            }
        }

        data[insert_index] = current_value;
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