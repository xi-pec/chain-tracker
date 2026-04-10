#![allow(non_snake_case)]

pub mod Deserialize;

pub unsafe fn init(image: &str, namespace: &str) {
    let class = "SingleModeBreedersLoadResponseFormatter";

    Deserialize::init(image, namespace, class);
}