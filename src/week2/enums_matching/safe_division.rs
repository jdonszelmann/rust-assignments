use weblab::weblab;

#[weblab(programming_assignment)]
/// In rust, dividing by zero results in a panic.
/// In this assignment, create a function `safe_division(i64, i64) -> Option<i64>` that returns an Option instead.
///
/// - In case the division is possible, return `Some(result)`.
/// - In case the division is not possible, return `None`
///
/// Next, create a function `safe_division_vec3(Vector3, Vector3) -> Option<Vector3>` that divides the components of the vectors element-wise.
/// This uses the Vector3 type from week 1. To implement `safe_division_vec3`, use the `safe_division` function and the `?` operator.
/// If any of the 3 element-wise divisions fails, return `None`.
#[weblab(title = "Safe Division")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        pub fn safe_division(a: i64, b: i64) -> Option<i64> {
            if b == 0 {
                None
            } else {
                Some(a / b)
            }
            //Or just a.checked_div(b)
        }

        /// Do not change the definition of Vector3!
        #[derive(Eq, PartialEq)]
        pub struct Vector3 {
            pub x: i64,
            pub y: i64,
            pub z: i64,
        }

        pub fn safe_division_vec3(a: Vector3, b: Vector3) -> Option<Vector3> {
            let x = safe_division(a.x, b.x)?;
            let y = safe_division(a.y, b.y)?;
            let z = safe_division(a.z, b.z)?;

            Some(Vector3 { x, y, z })
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        pub fn safe_division(a: i64, b: i64) -> Option<i64> {
            todo!()
        }

        /// Do not change the definition of Vector3!
        #[derive(Eq, PartialEq)]
        pub struct Vector3 {
            pub x: i64,
            pub y: i64,
            pub z: i64,
        }

        pub fn safe_division_vec3(a: Vector3, b: Vector3) -> Option<Vector3> {
            todo!()
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        #[test]
        fn test_div() {
            assert_eq!(safe_division(9, 3), Some(3));
        }

        solution_only! {
            #[test]
            fn test_division_nonzero() {
                assert_eq!(safe_division(9, 3), Some(3));
                assert_eq!(safe_division(-9, 3), Some(-3));
                assert_eq!(safe_division(1, -1), Some(-1));
                assert_eq!(safe_division(18, 4), Some(4));
                assert_eq!(safe_division(-7, 3), Some(-2));
            }

            #[test]
            fn test_division_zero() {
                assert_eq!(safe_division(9, 0), None);
                assert_eq!(safe_division(1, 0), None);
                assert_eq!(safe_division(0, 0), None);
                assert_eq!(safe_division(-1, 0), None);
                assert_eq!(safe_division(-9, 0), None);
            }

            fn vec3_check(v1: (i64, i64, i64), v2: (i64, i64, i64), v3: Option<(i64, i64, i64)>) {
                let vec1 = Vector3{ x: v1.0, y: v1.1, z: v1.2 };
                let vec2 = Vector3{ x: v2.0, y: v2.1, z: v2.2 };
                let vec3 = v3.map(|v3| Vector3{ x: v3.0, y: v3.1, z: v3.2 });
                let vec_div = safe_division_vec3(vec1, vec2);
                assert!(vec3 == vec_div);
            }

            #[test]
            fn test_division_vec3_nonzero() {
                vec3_check((3, 6, 9), (3, 3, 3), Some((1, 2, 3)));
                vec3_check((-3, 6, -9), (3, -3, 3), Some((-1, -2, -3)));
                vec3_check((7, 10, 54), (2, 3, 4), Some((3, 3, 13)));
            }

            #[test]
            fn test_division_vec3_zero() {
                vec3_check((7, -10, 54), (0, 3, 4), None);
                vec3_check((7, -10, 54), (3, 0, 3), None);
                vec3_check((7, -10, 54), (3, 3, 0), None);
                vec3_check((7, -10, 54), (0, 0, 3), None);
                vec3_check((7, -10, 54), (3, 0, 0), None);
                vec3_check((7, -10, 54), (0, 3, 0), None);
                vec3_check((7, -10, 54), (0, 0, 0), None);
            }
        }
    }
}
