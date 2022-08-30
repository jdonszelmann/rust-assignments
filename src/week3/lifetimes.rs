use weblab::weblab;

#[weblab(programming_assignment)]
/// # Lifetimes
///
/// The code in the template doesn't compile. However, it can be made to compile
/// by ***ONLY*** adding lifetime constraint annotations. It's your task to add these
/// annotations.
///
/// This item will be manually graded to ensure you did not change the code itself.
/// You should only add lifetime annotations.
///
/// A lifetime annotation is a marker after a reference type, making the lifetime of that reference
/// explicit, like `&'a u8` names the lifetime of this reference `'u8`.
#[weblab(title = "Lifetime")]
#[weblab(weight = 2)]
#[weblab(check = "only lifetimes were changed")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use std::borrow::Cow;

        pub fn swap<'a, 'b>(a: &'a str, b: &'b str, c: &mut &'b str, d: &mut &'a str) {
            *c = b;
            *d = a;
        }

        pub fn example<'a>(num: &usize, string_1: &'a str) -> Cow<'a, str> {
            let string_2 = format!("{}", num);
            let mut o1 = "";
            let mut o2 = "";

            swap(string_1, &string_2, &mut o1, &mut o2);

            if num % 2 == 0 {
                Cow::Borrowed(o2)
            } else {
                Cow::Owned(o1.to_string())
            }
        }
    }

    #[weblab(solution_template)]
    mod solution_template {
        use weblab::template_only;
        // this template_only! call is to "hide" to the compiler, the fact that the
        // code in it doesn't compile at all. Even when we're already in the solution_template,
        // rust still tries to parse this code, which it can't cause it requires lifetime bounds
        // as generics in some places.
        template_only! {
            use std::borrow::Cow;

            pub fn swap(a: &str, b: &str, c: &mut &str, d: &mut &str) {
                *c = b;
                *d = a;
            }

            pub fn example(num: &usize, string_1: &str) -> Cow<str> {
                let string_2 = format!("{}", num);
                let mut o1 = "";
                let mut o2 = "";

                swap(string_1, &string_2, &mut o1, &mut o2);

                if num % 2 == 0 {
                    Cow::Borrowed(o2)
                } else {
                    Cow::Owned(o1.clone())
                }
            }
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use std::borrow::Cow;
        use weblab::solution_only;

        solution_only! {
            fn test_with_lifetime<'a>(num: &usize, s: &'a str) -> Cow<'a, str> {
                example(num, s)
            }

            #[test]
            fn test_number() {
                assert_eq!(test_with_lifetime(&3, "test"), "3")
            }

            #[test]
            fn test_str() {
                assert_eq!(test_with_lifetime(&2, "test"), "test")
            }
        }
    }
}
