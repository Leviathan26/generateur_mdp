mod utils;
use std::env;
slint::include_modules!();

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // --- MODE GUI ---
        let ui = AppWindow::new().unwrap();
        // let ui_handle = ui.as_weak();

        ui.on_generer_clic({
            let ui_handle = ui.as_weak();
            move || {
                let ui = ui_handle.unwrap();
                let longueur = ui.get_longueur_mdp() as usize;
                let type_str = ui.get_type_selectionne();

                let t = match type_str.as_str() {
                    "Alphabetique" => utils::generator::TypeMdp::Alphabetique,
                    "Numerique" => utils::generator::TypeMdp::Numerique,
                    "Hexadecimal" => utils::generator::TypeMdp::Hexadecimal,
                    _ => utils::generator::TypeMdp::Mixte,
                };

                let mdp = utils::generator::generer_mot_de_passe(longueur, t);
                ui.set_mdp_affiche(mdp.into());
            }
        });

        let ui_handle_copy = ui.as_weak();
        ui.on_copier_clic(move || {
            let ui = ui_handle_copy.unwrap();
            let _ = utils::system::copier_au_presse_papier(&ui.get_mdp_affiche());
        });

        // --- AJOUTE CETTE LIGNE ICI ---
        let ui_handle_search = ui.as_weak();

        ui.on_rechercher_clic(move || {
            let ui = ui_handle_search.unwrap();
            let query = ui.get_recherche_query();

            match utils::system::rechercher_dans_fichier_gui(&query) {
                Ok(res) => ui.set_resultats_historique(res.into()),
                Err(e) => ui.set_resultats_historique(format!("Erreur : {}", e).into()),
            }
        });

        let ui_handle_save = ui.as_weak();
        ui.on_sauvegarder_clic(move || {
            let ui = ui_handle_save.unwrap();
            let mdp = ui.get_mdp_affiche();
            let label = ui.get_label_site();
            if label.trim().is_empty() {
                ui.set_couleur_statut(slint::Color::from_rgb_u8(200, 0, 0)); // Rouge
                ui.set_message_statut("Erreur : Label vide !".into());
                return;
            }
            match utils::system::sauvegarder_dans_fichier(&mdp, "GUI", &label) {
                Ok(_) => {
                    ui.set_mdp_affiche("Cliquer sur Générer".into());
                    ui.set_couleur_statut(slint::Color::from_rgb_u8(0, 150, 0)); // Vert
                    ui.set_message_statut("Sauvegarde réussie !".into());
                    ui.invoke_reset_focus(); 

                    // Faire disparaître le message après 3 secondes
                    let ui_timer = ui_handle_save.unwrap();
                    slint::Timer::single_shot(std::time::Duration::from_secs(3), move || {
                        ui_timer.set_message_statut("".into());
                    });
                }
                Err(e) => {
                    ui.set_message_statut(format!("Erreur : {}", e).into());
                }
            }
        });

        ui.run().unwrap();
    } else {
        // --- MODE CLI ---
        lancer_cli(args);
    }
}

fn lancer_cli(args: Vec<String>) {
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        afficher_aide(); // L'appel doit être ici !
        return;
    }

    if let Some(pos) = args.iter().position(|r| r == "-f" || r == "--find") {
        // 1. On crée une String vide "solide" qui restera en mémoire
        let defaut = String::new();

        // 2. On récupère soit l'argument, soit notre String solide
        let nom = args.get(pos + 1).unwrap_or(&defaut);

        // 3. Maintenant Rust est content, car 'defaut' vit jusqu'à la fin du bloc
        let _ = utils::system::rechercher_dans_fichier(nom);
        return;
    }

    let longueur = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(16);
    let mdp = utils::generator::generer_mot_de_passe(longueur, utils::generator::TypeMdp::Mixte);

    println!("Généré: {}", mdp);

    if args.contains(&String::from("-c")) {
        let _ = utils::system::copier_au_presse_papier(&mdp);
        println!("Copié !");
    }
}

fn afficher_aide() {
    println!(
        r#"
==========================================================
        GÉNÉRATEUR & GESTIONNAIRE DE MOTS DE PASSE
==========================================================
UTILISATION:
    generateur_mdp [LONGUEUR] [TYPE] [OPTIONS]

ARGUMENTS:
    LONGUEUR    Nombre de caractères souhaités (défaut: 16)
    TYPE        Jeu de caractères à utiliser:
                alpha  : Lettres uniquement (a-z, A-Z)
                num    : Chiffres uniquement (0-9)
                hex    : Hexadécimal (0-9, A-F)
                mixte  : Lettres, chiffres et symboles (défaut)

OPTIONS:
    -c, --copy          Copie le mot de passe généré dans le presse-papier.
    
    -s, --save [NOM]    Sauvegarde le mot de passe dans 'historique_mdp.txt'
                        avec le NOM associé (ex: google.com, facebook, etc.).
    
    -f, --find [NOM]    Recherche et affiche les entrées correspondant au NOM
                        dans votre historique.
    
    -h, --help          Affiche ce menu d'aide.

EXEMPLES:
    Génération simple (affichage seul):
        generateur_mdp 20 alpha
    
    Générer et Copier:
        generateur_mdp 16 mixte -c
    
    Générer, Sauvegarder et Copier:
        generateur_mdp 24 hex --save amazon.fr --copy
    
    Rechercher un mot de passe enregistré:
        generateur_mdp --find amazon

NOTE: 
    Par défaut, le mot de passe n'est PAS copié si l'option -c n'est pas présente.
==========================================================
"#
    );
}
