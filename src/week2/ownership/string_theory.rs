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
        /// - A String can be converted to a `&str` by taking a reference to is it which is fast, whereas converting `&str` to `String` requires allocating space for it
        /// on the heap and copying the characters. So choosing `&str` as input type makes our function work on the largest amount of types without cloning.
    )
    open_question!(
        title: "Why return String",
        question:
        /// Explain why it makes sense for this function to return `String`, rather than `&str`
        answer:
        /// The function constructs a new String. This size of this string cannot be known at compile time, because it depends on the sizes
        /// of both the input strings. So at runtime, the right size needs to be found, allocated, and the inputs need to be copied over to be
        /// concatenated. A right type for an owned runtime-sized piece of text is `String`.
    )
}
