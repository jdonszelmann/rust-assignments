use weblab::{weblab_folder, weblab_main};

mod week1;
mod week2;
mod week3;
mod week4;

weblab_folder!("weekly assignments", week1, week2, week3, week4,);

fn main() {
    weblab_main!(self);
}
