use weblab::inline_question_list;

inline_question_list! {
    title: "String Theory",
    weight: 3.0,
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
        weight: 1.0,
        checklist: [
            "provide a good reason (see reference answer)"
        ],
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
        weight: 1.0,
        checklist: [
            "correct explanation that the length is unknown and has to be heap allocated or that ownership has to be transferred back to the caller"
        ],
        question:
        /// Explain why it makes sense for this function to return `String`, rather than `&str`
        answer:
        /// The function constructs a new String. This size of this string cannot be known at compile time, because it depends on the sizes
        /// of both the input strings. So at runtime, the right size needs to be found, allocated, and the inputs need to be copied over to be
        /// concatenated. A right type for an owned runtime-sized piece of text is `String`.
    )

    open_question!(
        title: "str",
        weight: 2.0,
        checklist: [
            "answer question 1 (difference)"
            "answer question 2 (both exists)"
        ],
        question:
        ///  Explain the difference between `str` and `&str`. Why can't we create variables of type str? And instead have to use &str?
        answer:
        /// A `str` is an *unsized* type. It's the size of a sequence of characters of unknown length.
        /// Because the length isn't known, we can't have a variable with `str` as type, because the
        /// stack size cannot be known when the program is compiled. However, we can wrap `str` in something
        /// to make it have a known size. For example, `&str` is a reference to a string, and references
        /// always have the same size. But you can conceivably also create a `&mut str`, `Box<str>`, or any
        /// other wrapper that makes its content a known size. A `String` is internally a `Box<str>`.
        ///
        /// When storing strings in variables, the type will therefore almost always store a &str. `str`
        /// is just a placeholder type to be wrapped in something.
    )
}
