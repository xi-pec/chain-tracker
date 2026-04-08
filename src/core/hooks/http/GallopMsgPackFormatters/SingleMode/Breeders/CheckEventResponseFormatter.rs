#![allow(non_snake_case)]

pub mod Deserialize;

pub unsafe fn init(image: &str, namespace: &str) {
    let klass = "SingleModeBreedersCheckEventResponseFormatter";

    Deserialize::init(image, namespace, klass);
}