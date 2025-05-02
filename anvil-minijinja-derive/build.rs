fn main() {
    if cfg!(test) {
        println!("cargo:rerun-if-changed=tests/templates");
        minijinja_embed::embed_templates!("tests/templates/");
    }
}
