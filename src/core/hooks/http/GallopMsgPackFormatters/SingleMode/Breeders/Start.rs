#![allow(non_snake_case)]

pub mod Deserialize;

pub unsafe fn init(image: &str, namespace: &str) {
    let class = "SingleModeBreedersStartResponseFormatter";

    Deserialize::init(image, namespace, class);
}