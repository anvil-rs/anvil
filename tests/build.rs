fn main() {
    println!("cargo:rerun-if-changed=templates");
    minijinja_embed::embed_templates!("templates");
}

