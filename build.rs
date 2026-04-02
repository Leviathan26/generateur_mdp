fn main() {
    // 1. Compile l'interface Slint
    slint_build::compile("src/main.slint").expect("Erreur Slint");

    // 2. Intègre l'icône au fichier .exe (uniquement sur Windows)
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        embed_resource::compile("icon.rc", embed_resource::NONE);
    }
}
