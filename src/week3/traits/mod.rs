use weblab::weblab_folder;

mod built_in_traits;
mod increment;
mod shapes;

weblab_folder!("traits", sum, shapes, increment, built_in_traits);
