use weblab::inline_question_list;

inline_question_list! {
    title: "Is it sized?",
    question:
    /// Does this type have a size known at compile time? In other words, is this type `Sized`?
    mc_question! {
        title: "1",
        question:
        /// `u8`
        explanation:
        /// A single byte has a known size! It's always 8 bits.
        option "yes, it's `Sized`" correct,
        option "no, it's not `Sized`"
    }
    mc_question! {
        title: "2",
        question:
        /// `i64`
        explanation:
        /// Even if the size is larger, the size is still known by the compiler.
        option "yes, it's `Sized`" correct,
        option "no, it's not `Sized`"
    }
    mc_question! {
        title: "3",
        question:
        /// `usize`
        explanation:
        /// Even though it depends on the system what the size of a usize is (it's 32 bits on a 32 bit system
        /// and 64 bit on a 64 bit system), the compiler still knows the size during compilation. The compiler
        /// compiles the code for a specific machine, so the compiler knows this size for the specific machine
        /// that it compiles for.
        option "yes, it's `Sized`" correct,
        option "no, it's not `Sized`"
    }
    mc_question! {
        title: "4",
        question:
        /// `[u8]`
        explanation:
        /// `[u8]` is not `Sized`. The compiler doesn't always know how much space to create on the
        /// stack to store an array of bytes. That depends on the length, which can change at runtime.
        option "yes, it's `Sized`",
        option "no, it's not `Sized`" correct,
    }
    mc_question! {
        title: "5",
        question:
        /// `&[u8]`
        explanation:
        /// A reference to any type is `Sized`, even if the type that is referenced is not `Sized`.
        /// That means that even though `[u8]` is not `Sized`, `&u8` *is*.
        option "yes, it's `Sized`" correct,
        option "no, it's not `Sized`",
    }
    mc_question! {
        title: "6",
        question:
        /// `&mut i64`
        explanation:
        /// A reference to any type is `Sized`. Even a reference to integers. The fact that it is
        /// mutable doesn't change the size either.
        option "yes, it's `Sized`" correct,
        option "no, it's not `Sized`",
    }
    mc_question! {
        title: "7",
        question:
        /// `str`
        explanation:
        /// The type `str` is similar to `[u8]`. It's size is not known until runtime so the
        /// compiler can't know for sure how much space to allocate for a `str` at compile time.
        option "yes, it's `Sized`",
        option "no, it's not `Sized`" correct,
    }
    mc_question! {
        title: "8",
        question:
        /// `&str`
        explanation:
        /// Again, a reference to any type is `Sized`.
        option "yes, it's `Sized`" correct,
        option "no, it's not `Sized`",
    }
    mc_question! {
        title: "9",
        question:
        /// `Box<str>`
        explanation:
        /// A Box represents an allocation on the heap. It's therefore pretty much a special kind of reference
        /// (it's special because it deallocates the object from the heap automatically). Because it's
        /// a reference, it's `Sized`
        option "yes, it's `Sized`" correct,
        option "no, it's not `Sized`",
    }
    mc_question! {
        title: "10",
        question:
        /// `String`
        explanation:
        /// The type `String` is essentially an alias of `Box<str>`. It's a heap allocated reference
        /// to a string. Thus, it's `Sized`
        option "yes, it's `Sized`" correct,
        option "no, it's not `Sized`",
    }
    mc_question! {
        title: "11",
        question:
        /// `&Box<&[u8]>`
        explanation:
        /// Adding more references doesn't matter! It's still stored as pretty much a pointer, and is
        /// thus `Sized`.
        option "yes, it's `Sized`" correct,
        option "no, it's not `Sized`",
    }
}
