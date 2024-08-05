pub mod mamoru;
pub mod mamoru_serialize;
pub mod mamoru_storage;
pub mod sui_ctx;

use wit_bindgen::generate;

generate!({
    world: "mamoru-core",
    path: "wit/components.wit",
});
