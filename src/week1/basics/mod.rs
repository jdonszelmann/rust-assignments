use weblab::weblab_folder;

mod arithmetic;
mod luhn;
mod numeric_properties;
mod quadratics;

weblab_folder!("basics", arithmetic, numeric_properties, quadratics, luhn);
