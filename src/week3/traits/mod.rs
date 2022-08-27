use weblab::weblab_folder;

mod built_in_traits;
mod increment;
mod shapes;

weblab_folder!("traits", shapes, increment);
