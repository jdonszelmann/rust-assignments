use weblab::weblab_folder;

mod increment;
mod shapes;
mod tree_iterator;
mod built_in_traits;

weblab_folder!("traits", shapes, increment, tree_iterator);
