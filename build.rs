fn main() {
    tonic_build::configure()
        .compile(&["proto/kvstore.proto"], &["proto"])
        .unwrap();

    println!("build kv store proto");
}
