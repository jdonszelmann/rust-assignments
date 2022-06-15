use weblab::inline_question_list;

inline_question_list! {
    title: "String Theory",
    question:
    /// In the previous assignment, you were asked to implement the following function:
    /// ```
    /// pub fn concat(a: &str, b: &str) -> String {
    ///     todo!()
    /// }
    /// ```
    ///
    /// If you don't know the answer to either of these questions, try it out!

    open_question!(
        title: "Why receive &str",
        question:
        /// Explain why it makes sense for this function to receive `&str` as an argument, rather than `String`
        answer:
        /// Some good reasons:
        /// - The function does not need ownership of `a` and `b`, since it only reads them.
        /// - A String can be converted to a `&str` efficiently, whereas converting `&str` to `String` requires cloning. So choosing `&str` as input type makes our function the most useful.
    )
    open_question!(
        title: "Why return String",
        question:
        /// Explain why it makes sense for this function to return `String`, rather than `&str`
        answer:
        /// It is not possible to return `&str` without leaking memory, unless the returned `&str` would point into one of the input `&str`s or it would return a `&'static str`, neither of which is possible given the function specification.
    )
}
