use weblab::weblab_folder;

mod increment;
mod shapes;
// mod custom_iterator;
mod tree_iterator;

weblab_folder!("traits", shapes, increment, /* custom_iterator,*/ tree_iterator);
