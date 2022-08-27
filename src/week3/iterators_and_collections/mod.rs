mod case_converter;
mod exercises;
mod hash_collections;
mod ringbuffer;
mod tree_iterator;

use weblab::weblab_folder;

weblab_folder!(
    "Iterators and Collections",
    exercises,
    tree_iterator,
    case_converter,
    hash_collections,
    ringbuffer,
);
