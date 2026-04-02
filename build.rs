fn main() {
    // 1. Compile l'interface Slint
    slint_build::compile("src/main.slint").expect("Erreur Slint");

    // 2. Métadonnées et Icône Windows
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("icon.ico"); // Ton icône
        res.set("Language", "0x040c"); // Français
        res.set("CompanyName", "NDSOFTKIT");
        res.set("FileDescription", "Coffre-fort GesPass : Générateur et Gestionnaire de mots de passe sécurisé");
        res.set("LegalCopyright", "Copyright © 2024-2026 NDSOFTKIT. Tous droits réservés.");
        res.set("Comments", "Contact : ndsoftkit@gmail.com");
        res.compile().expect("Erreur lors de la compilation des ressources");
    }
}
