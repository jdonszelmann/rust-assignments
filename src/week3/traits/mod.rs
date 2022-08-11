use weblab::weblab_folder;

mod built_in_traits;
mod increment;
mod shapes;
mod tree_iterator;

weblab_folder!("traits", shapes, increment, tree_iterator);
