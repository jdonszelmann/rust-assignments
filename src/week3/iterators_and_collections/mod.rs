mod case_converter;
mod exercises;
mod hash_collections;
mod ringbuffer;

use weblab::weblab_folder;

weblab_folder!(
    "Iterators and Collections",
    exercises,
    case_converter,
    hash_collections,
    ringbuffer,
);
