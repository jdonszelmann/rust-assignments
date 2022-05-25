use weblab::inline_question_list;

inline_question_list! {
   title: "ownership rules"

    open_question!(
        title: "explain the example",
        question:
        /// Explain why the following code does not work.
        ///
        /// ```rust
        /// fn do_something(a: Vec<u64>) {
        ///     // ...
        /// }
        ///
        /// fn main() {
        ///     let v = vec![1, 2, 3];
        ///     do_something(a);
        ///
        ///     println!("{:?}", a);
        /// }
        /// ```
        answer:
        /// In the call to `do_something`, the ownership of the vector in `v` is moved into the do_something function.
        /// After this call the `do_something` function will deallocate the vector.
        ///
        /// This means that we cannot print a after the call to `do_something`. The compiler will error when it sees this.
    )

    open_question!(
        title: "solutions",
        question:
        /// Explain two strategies to fix the code from the previous question.
        answer:
        /// # Solution 1
        /// Change `do_something` to take a reference to a Vector. This means the function will only
        /// borrow the vector in `v`, and thus it will still be available after the call.
        ///
        /// ```rust
        /// fn do_something(a: &Vec<u64>) {
        ///     // ...
        /// }
        ///
        /// fn main() {
        ///     let v = vec![1, 2, 3];
        ///     do_something(&a);
        ///
        ///     println!("{:?}", a);
        /// }
        /// ```
        ///
        /// # Solution 2
        ///
        /// Clone the vector. Now the `do_something` function will get its own copy of the vector,
        /// and the one in `main` will still be available. Note that one of the copies of the vector
        /// will still be deallocated at the end of `do_something`. But because it's a copy, the version
        /// in main is not affected.
        ///
        /// ```rust
        /// fn do_something(a: Vec<u64>) {
        ///     // ...
        /// }
        ///
        /// fn main() {
        ///     let v = vec![1, 2, 3];
        ///     do_something(a.clone());
        ///
        ///     println!("{:?}", a);
        /// }
        /// ```
    )
}
