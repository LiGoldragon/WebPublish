fn main() {
    capnpc::CompilerCommand::new()
        .file("webpublish.capnp")
        .run()
        .expect("Cap'n Proto schema compilation failed");
}
