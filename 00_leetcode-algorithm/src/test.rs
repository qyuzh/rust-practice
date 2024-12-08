#[macro_export]
macro_rules! arr_to_vec {
    ($arr:expr) => {{
        let mut vec = Vec::new();
        for element in $arr {
            vec.push(element);
        }
        vec
    }};
}

#[macro_export]
macro_rules! arr2d_to_vec2d {
    ($arr:expr) => {{
        let mut vec = Vec::new();
        for row in $arr {
            let mut vec2 = Vec::new();
            for element in row {
                vec2.push(element);
            }
            vec.push(vec2);
        }
        vec
    }};
}

#[cfg(test)]
mod tests {

    #[test]
    fn tests1() {
        let t = arr2d_to_vec2d!([[1, 2, 3], [4, 5, 6]]);
        assert_eq!(t, vec![vec![1, 2, 3], vec![4, 5, 6]]);
    }

    #[test]
    fn tests2() {
        let t = arr_to_vec!([1, 2, 3]);
        assert_eq!(t, vec![1, 2, 3])
    }
}
