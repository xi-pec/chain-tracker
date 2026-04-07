#![allow(non_snake_case)]

pub mod GallopMsgPackFormatters;

pub unsafe fn init() {
    let image = "umamusume.Http";

    GallopMsgPackFormatters::init(image);
}