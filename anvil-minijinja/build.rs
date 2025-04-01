// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=templates");
    minijinja_embed::embed_templates!("templates/");
}
