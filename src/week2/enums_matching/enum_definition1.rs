use weblab::weblab;

#[weblab(programming_assignment)]
/// For this assignment, create an enum `WeekDay` that represents a day of the week.
/// The days should be spelled exactly:
/// `Monday, Tuesday, Wednesday, Thursday, Friday, Saturday, Sunday`
///
/// Then implement a function on `WeekDay` called `next`, that *mutates* the week day and changes it to the next week day.
/// For example, if it is currently Wednesday. Calling `next` once makes it Thursday.
///
/// Finally, implement a function on WeekDay called `wait_time`.
/// It answers the question "if it is currently week day `self`, and I have to wait until week day `until`, how many days do I have to wait?
///
/// For example, if it is Tuesday and I have to wait until Sunday, I have to wait 5 days. If it Friday and I have to wait until Friday, I have to wait 0 days.
/// So `WeekDay::Tuesday.wait_time(WeekDay::Sunday)` should be 5.
#[weblab(title = "Simple Enum Definitions")]
#[weblab(weight=2)]
mod assignment {
    #[weblab(solution)]
    mod solution {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        pub enum WeekDay {
            Monday,
            Tuesday,
            Wednesday,
            Thursday,
            Friday,
            Saturday,
            Sunday,
        }

        impl WeekDay {
            pub fn next(&mut self) {
                *self = match self {
                    WeekDay::Monday => WeekDay::Tuesday,
                    WeekDay::Tuesday => WeekDay::Wednesday,
                    WeekDay::Wednesday => WeekDay::Thursday,
                    WeekDay::Thursday => WeekDay::Friday,
                    WeekDay::Friday => WeekDay::Saturday,
                    WeekDay::Saturday => WeekDay::Sunday,
                    WeekDay::Sunday => WeekDay::Monday,
                }
            }

            pub fn wait_time(&self, until: Self) -> u8 {
                let mut day = *self;
                let mut counter = 0;

                while day != until {
                    counter += 1;
                    day.next();
                }

                counter
            }
        }
    }

    #[allow(unused_variables)]
    #[weblab(solution_template)]
    mod solution_template {
        #[derive(Copy, Clone, Eq, PartialEq, Debug)]
        ///TODO add variants
        pub enum WeekDay {}

        impl WeekDay {
            pub fn next(&mut self) {
                todo!()
            }

            pub fn wait_time(&self, until: Self) -> u8 {
                todo!()
            }
        }
    }

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        #[test]
        pub fn test_weekdays() {
            let _ = WeekDay::Monday;
            let _ = WeekDay::Tuesday;
            let _ = WeekDay::Wednesday;
            let _ = WeekDay::Thursday;
            let _ = WeekDay::Friday;
            let _ = WeekDay::Saturday;
            let _ = WeekDay::Sunday;
        }

        #[test]
        pub fn next_weekday_example() {
            let mut day = WeekDay::Wednesday;
            day.next();
            assert_eq!(day, WeekDay::Thursday);
        }

        #[test]
        pub fn wait_time_example() {
            assert_eq!(WeekDay::Tuesday.wait_time(WeekDay::Sunday), 5);
            assert_eq!(WeekDay::Friday.wait_time(WeekDay::Friday), 0);
        }

        solution_only! {
            #[test]
            pub fn test_all_weekdays_next() {
                let mut day = WeekDay::Wednesday;

                day.next();
                assert_eq!(day, WeekDay::Thursday);

                day.next();
                assert_eq!(day, WeekDay::Friday);

                day.next();
                assert_eq!(day, WeekDay::Saturday);

                day.next();
                assert_eq!(day, WeekDay::Sunday);

                day.next();
                assert_eq!(day, WeekDay::Monday);

                day.next();
                assert_eq!(day, WeekDay::Tuesday);

                day.next();
                assert_eq!(day, WeekDay::Wednesday);
            }

            #[test]
            pub fn test_full_wait_time() {
                assert_eq!(WeekDay::Monday.wait_time(WeekDay::Monday), 0);
                assert_eq!(WeekDay::Monday.wait_time(WeekDay::Tuesday), 1);
                assert_eq!(WeekDay::Monday.wait_time(WeekDay::Wednesday), 2);
                assert_eq!(WeekDay::Monday.wait_time(WeekDay::Thursday), 3);
                assert_eq!(WeekDay::Monday.wait_time(WeekDay::Friday), 4);
                assert_eq!(WeekDay::Monday.wait_time(WeekDay::Saturday), 5);
                assert_eq!(WeekDay::Monday.wait_time(WeekDay::Sunday), 6);

                assert_eq!(WeekDay::Thursday.wait_time(WeekDay::Sunday), 3);
                assert_eq!(WeekDay::Thursday.wait_time(WeekDay::Monday), 4);
                assert_eq!(WeekDay::Friday.wait_time(WeekDay::Wednesday), 5);
                assert_eq!(WeekDay::Saturday.wait_time(WeekDay::Sunday), 1);
                assert_eq!(WeekDay::Sunday.wait_time(WeekDay::Thursday), 4);
                assert_eq!(WeekDay::Sunday.wait_time(WeekDay::Saturday), 6);
            }
        }
    }
}
