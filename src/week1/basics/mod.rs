use weblab::weblab_folder;

mod arithmetic;
mod fibonacci;
mod luhn;
mod numeric_properties;
mod quadratics;

weblab_folder!(
    "basics",
    sum,
    arithmetic,
    numeric_properties,
    quadratics,
    luhn,
    fibonacci
);
