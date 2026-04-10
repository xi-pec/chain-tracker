#![allow(non_snake_case)]

pub mod SingleMode;

pub unsafe fn init(image: &str) {
    let namespace = "Gallop.MsgPack.Formatters";

    SingleMode::Onsen::CheckEvent::init(image, namespace);
    SingleMode::Onsen::ExecCommand::init(image, namespace);
    SingleMode::Onsen::Load::init(image, namespace);
    SingleMode::Onsen::Start::init(image, namespace);
    
    SingleMode::Breeders::CheckEvent::init(image, namespace);
    SingleMode::Breeders::ExecCommand::init(image, namespace);
    SingleMode::Breeders::Load::init(image, namespace);
    SingleMode::Breeders::Start::init(image, namespace);
}