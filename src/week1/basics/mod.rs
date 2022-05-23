use weblab::weblab_folder;

mod arithmetic;
mod numeric_properties;
mod luhn;

weblab_folder!("basics", arithmetic, numeric_properties, luhn);
