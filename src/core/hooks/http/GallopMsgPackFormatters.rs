#![allow(non_snake_case)]

pub mod SingleMode;

pub unsafe fn init(image: &str) {
    let namespace = "Gallop.MsgPack.Formatters";

    SingleMode::Onsen::ExecCommandResponseFormatter::init(image, namespace);
    SingleMode::Onsen::CheckEventResponseFormatter::init(image, namespace);
    SingleMode::Breeders::ExecCommandResponseFormatter::init(image, namespace);
    SingleMode::Breeders::CheckEventResponseFormatter::init(image, namespace);
}