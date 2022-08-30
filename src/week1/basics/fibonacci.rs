use weblab::weblab;

#[weblab(programming_assignment)]
/// # Fibonacci
/// A popular mathematical sequence is the Fibonacci sequence.
/// The Fibonacci sequence is defined mathematically as follows for any non-negative integer $n$:
/// $$
/// F_0 = 0
/// $$
/// $$
/// F_1 = 1
/// $$
/// $$
/// F_n = F_{n-1} + F_{n-2}
/// $$
///
/// This means we can find the $n$-th Fibonacci number by adding the previous two.
/// E.g. we find that
/// $$
/// F_4 = 3
/// $$
/// $$
/// F_8 = 21
/// $$
/// $$
/// F_{12} = 144
/// $$
///
/// ## Assignment
/// Create a function `fibonacci(n: u64) -> u64;` that computes and returns (in a reasonable time) the `n`th fibonacci number for any `n`.
#[weblab(title = "fibonacci")]
#[weblab(weight = 2)]
mod assignment {
    #[weblab(solution)]
    mod solution {
        // pub fn fibonacci(n: u64) -> u64 {
        //     match n {
        //         0 => 0,
        //         1 => 1,
        //         n => fibonacci(n - 1) + fibonacci(n - 2)
        //     }
        // }

        pub fn fibonacci(n: u64) -> u64 {
            if n == 0 {
                return 0;
            };

            let mut temp1;
            let mut temp2 = 0;
            let mut curr = 1;
            for _ in 2..=n {
                temp1 = temp2;
                temp2 = curr;

                curr = temp1 + temp2;
            }

            curr
        }
    }

    #[weblab(solution_template)]
    #[allow(unused_variables)]
    mod solution_template {
        use weblab::solution_only;

        pub fn fibonacci(n: u64) -> u64 {
            todo!()
        }
    }

    #[weblab(test_template)]
    mod test_template {
        use super::solution::fibonacci;

        #[test]
        fn test_fibonacci_4() {
            assert_eq!(fibonacci(4), 3);
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::fibonacci;
        use std::sync::mpsc;
        use std::thread;
        use std::time::Duration;

        fn fibonacci_ref(n: u64) -> u64 {
            if n == 0 {
                return 0;
            };

            let mut temp1;
            let mut temp2 = 0;
            let mut curr = 1;
            for _ in 2..=n {
                temp1 = temp2;
                temp2 = curr;

                curr = temp1 + temp2;
            }

            curr
        }

        fn panic_after<T, F>(d: Duration, f: F) -> T
        where
            T: Send + 'static,
            F: FnOnce() -> T,
            F: Send + 'static,
        {
            let (done_tx, done_rx) = mpsc::channel();
            let handle = thread::spawn(move || {
                let val = f();
                done_tx.send(()).expect("Unable to send completion signal");
                val
            });

            match done_rx.recv_timeout(d) {
                Ok(_) => handle.join().expect("Thread panicked"),
                Err(_) => panic!("Thread took too long"),
            }
        }

        #[test]
        fn test_0() {
            assert_eq!(fibonacci(0), 0);
        }

        #[test]
        fn test_1() {
            assert_eq!(fibonacci(1), 1);
        }

        #[test]
        fn test_2() {
            assert_eq!(fibonacci(2), 1);
        }

        #[test]
        fn test_3() {
            assert_eq!(fibonacci(3), 2);
        }

        #[test]
        fn test_4() {
            assert_eq!(fibonacci(4), 3);
        }

        #[test]
        fn test_8() {
            panic_after(Duration::from_secs_f64(5.0), || {
                assert_eq!(fibonacci(8), 21);
            });
        }

        #[test]
        fn test_12() {
            panic_after(Duration::from_secs_f64(5.0), || {
                assert_eq!(fibonacci(12), 144);
            });
        }

        #[test]
        fn test_13() {
            panic_after(Duration::from_secs_f64(5.0), || {
                assert_eq!(fibonacci(90), fibonacci_ref(90));
            });
        }
    }
}
