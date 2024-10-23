#![allow(non_snake_case)]

// define modules
mod pages;
mod route;
mod app;
mod server;

use crate::app::launch_app;

fn main() {
    launch_app();
}
