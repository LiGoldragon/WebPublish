fn main() {
    capnpc::CompilerCommand::new()
        .file("../../webpublish.capnp")
        .output_path("src")
        .run()
        .expect("Cap'n Proto schema compilation failed");
}
