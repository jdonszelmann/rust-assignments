use weblab::weblab;

#[weblab(programming_assignment)]
/// A ringbuffer is a datastructure based on an array (or a Vec).
/// They are often used as a queue for data. For example, holding keystrokes
/// from a keyboard until time is available to handle them.
///
/// However, compared to arbitrary-size queue datastructures, a ringbuffer used a fixed
/// amount of memory. When the buffer is full, the oldest value in the queue is simply
/// overwritten, and thus lost (though some implementations may alternatively error in such a case).
///
/// A ringbuffer works with two pointers (though you may implement them as just indices in the array).
/// One marks the end of data, while the other marks the start. The data in the ringbuffer is everything
/// from `start` to `end`. However, this range may *wrap around* the end of the array. For example:
/// ```
///                     V unused (the queue is not full)
/// [1, 2, 3, 1, 3, _, _, _, _, 1, 2, 3, 3, 2] -> wrap around
///              ^ end          ^ start
/// ```
///
/// To achieve this wrapping behavior, often, the remainder operation is used.
///
/// For this assignment:
/// * Make a `struct Ringbuffer<T>`, *which is generic* over a type T, so it can store arbitrary data.
///     * In tests, we use different integer types for T. You may thus bound T to be `Clone` and `Eq` if that's required for your implementation
/// * implement an associated function `fn new(size: usize) -> Ringbuffer<T>` that creates a new ringbuffer of a specific size.
/// * implement a method `fn enqueue(&mut self, v: T) -> Option<T>` which optionally returns the value overwritten by this enqueue if the buffer is full
/// * implement a method `fn dequeue(&mut self) -> Option<T>` which returns the next value in the queue
/// * implement a method `fn peek(&self) -> Option<&T>` which returns a reference to the value that would be dequeued next (but doesn't actually remove it)
/// * implement a method `fn len(&self) -> usize` which returns the number of elements in the queue.
///
/// Make sure that both the struct and it's methods are declared `pub` (public) since we can't test them otherwise.
///
/// The actual implementation of the ringbuffer, and its internals are up to you. There is more than one correct answer.
#[weblab(title = "Ringbuffer")]
mod assignment {
    #[weblab(solution)]
    mod solution {
        use std::cmp::Ordering;

        pub struct RingBuffer<T> {
            buffer: Vec<Option<T>>,
            read: usize,
            write: usize,
        }

        impl<T> RingBuffer<T> {
            pub fn new(size: usize) -> Self {
                let mut buffer = Vec::with_capacity(size);
                buffer.resize_with(size, || None);

                Self {
                    buffer,
                    read: 1,
                    write: 0,
                }
            }

            pub fn len(&self) -> usize {
                self.write + 1 - self.read
            }

            pub fn enqueue(&mut self, v: T) -> Option<T> {
                if self.buffer.is_empty() {
                    return None;
                }

                let return_value = (self.len() == self.buffer.len())
                    .then(|| self.dequeue())
                    .flatten();

                self.write += 1;
                let index = self.write % self.buffer.len();
                self.buffer[index] = Some(v);

                return_value
            }

            pub fn dequeue(&mut self) -> Option<T> {
                if self.write >= self.read {
                    let index = self.read % self.buffer.len();
                    let result = self.buffer[index].take();
                    self.read += 1;

                    result
                } else {
                    None
                }
            }

            pub fn peek(&self) -> Option<&T> {
                if self.write >= self.read {
                    let index = self.read % self.buffer.len();
                    self.buffer[index].as_ref()
                } else {
                    None
                }
            }
        }
    }

    #[weblab(solution_template)]
    mod solution_template {}

    #[weblab(test)]
    mod test {
        use super::solution::*;
        use weblab::{solution_only, template_only};

        #[test]
        fn smoke() {
            let mut rb = RingBuffer::new(10);
            for i in 0..20 {
                rb.enqueue(i);
            }

            assert_eq!(rb.len(), 10);
        }

        solution_only! {
            #[test]
            fn empty_length() {
                assert_eq!(RingBuffer::<i32>::new(10).len(), 0)
            }

            #[test]
            fn one_length() {
                let mut rb = RingBuffer::new(10);
                rb.enqueue(5);
                assert_eq!(rb.len(), 1);
            }

            #[test]
            fn enqueue_dequeue_one() {
                let mut rb = RingBuffer::new(10);
                rb.enqueue(5);
                assert_eq!(rb.dequeue(), Some(5));
            }

            #[test]
            fn enqueue_dequeue_two() {
                let mut rb = RingBuffer::new(10);
                rb.enqueue(5);
                rb.enqueue(6);
                assert_eq!(rb.dequeue(), Some(5));
                assert_eq!(rb.dequeue(), Some(6));
            }

            #[test]
            fn enqueue_dequeue_almost_all() {
                let mut rb = RingBuffer::new(10);
                for i in 0..9 {
                    rb.enqueue(i * 10);
                }
                for i in 0..9 {
                    assert_eq!(rb.dequeue(), Some(i * 10));
                }
            }

            #[test]
            fn enqueue_dequeue_all() {
                let mut rb = RingBuffer::new(10);
                for i in 0..10 {
                    println!("insert {}", i * 10);
                    rb.enqueue(i * 10);
                }
                for i in 0..10 {
                    assert_eq!(rb.dequeue(), Some(i * 10));
                }
            }

            #[test]
            fn enqueue_dequeue_all_len() {
                let mut rb = RingBuffer::new(10);
                for i in 0..10 {
                    println!("insert {}", i * 10);
                    assert_eq!(rb.len(), i);
                    rb.enqueue(i * 10);
                }
                assert_eq!(rb.len(), 10);
                for i in 0..10 {
                    assert_eq!(rb.dequeue(), Some(i * 10));
                }
                assert_eq!(rb.len(), 0);
            }

            #[test]
            fn dequeue_after_enqueue_too_much() {
                let mut rb = RingBuffer::new(10);
                for i in 0..20 {
                    println!("insert {}", i * 10);
                    rb.enqueue(i * 10);
                }
                for i in 10..20 {
                    assert_eq!(rb.dequeue(), Some(i * 10));
                }
            }

            #[test]
            fn enqueue_dequeues_first_when_full() {
                let mut rb = RingBuffer::new(10);
                for i in 0..10 {
                    assert!(rb.enqueue(i).is_none());
                }
                assert_eq!(rb.enqueue(42), Some(0));
                assert_eq!(rb.enqueue(42), Some(1));
                assert_eq!(rb.enqueue(42), Some(2));
            }

            #[test]
            fn dequeue_empty() {
                let mut rb = RingBuffer::<i32>::new(10);
                assert_eq!(rb.dequeue(), None)
            }

            #[test]
            fn zero_size() {
                let mut rb = RingBuffer::<i32>::new(0);
                assert_eq!(rb.enqueue(5), None);
                assert_eq!(rb.dequeue(), None);
                assert_eq!(rb.peek(), None);
                assert_eq!(rb.len(), 0);
            }

            #[test]
            fn one_size() {
                let mut rb = RingBuffer::<i32>::new(1);
                assert_eq!(rb.len(), 0);
                rb.enqueue(0);
                assert_eq!(rb.len(), 1);
                for i in 0..1000 {
                    assert_eq!(rb.enqueue(i + 1), Some(i));
                }
                assert_eq!(rb.len(), 1);
                assert_eq!(rb.dequeue(), Some(1000));
                assert_eq!(rb.len(), 0);
            }

            #[test]
            fn peek_empty() {
                assert_eq!(RingBuffer::<i32>::new(10).peek(), None);
            }

            #[test]
            fn peek_one() {
                let mut rb = RingBuffer::<i32>::new(10);
                rb.enqueue(10);
                assert_eq!(rb.peek(), Some(&10));
            }

            #[test]
            fn peek_more() {
                let mut rb = RingBuffer::<i32>::new(10);
                rb.enqueue(10);
                rb.enqueue(11);
                assert_eq!(rb.peek(), Some(&10));
            }

            #[test]
            fn peek_after_dequeue() {
                let mut rb = RingBuffer::<i32>::new(10);
                rb.enqueue(10);
                rb.enqueue(11);
                rb.dequeue();
                assert_eq!(rb.peek(), Some(&11));
            }

            #[test]
            fn peek_multiple_times() {
                let mut rb = RingBuffer::<i32>::new(10);
                rb.enqueue(10);
                assert_eq!(rb.peek(), Some(&10));
                assert_eq!(rb.peek(), Some(&10));
            }

            #[test]
            fn peek_after_reempty() {
                let mut rb = RingBuffer::<i32>::new(10);
                rb.enqueue(10);
                rb.enqueue(11);
                rb.dequeue();
                rb.dequeue();
                assert_eq!(rb.peek(), None);
            }
        }
    }
}
