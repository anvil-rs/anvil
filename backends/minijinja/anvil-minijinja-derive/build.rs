fn main() {
    println!("cargo:rerun-if-changed=tests/templates");
    minijinja_embed::embed_templates!("tests/templates/");
}
