extern crate embed_resource;

fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        static_vcruntime::metabuild();

        println!("cargo:rerun-if-changed=app-name-manifest.rc");
        embed_resource::compile("gui-launcher-manifest.rc", embed_resource::NONE)
    }
}
