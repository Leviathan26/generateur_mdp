pub mod system {
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    use arboard::Clipboard;
    use rand::RngCore;
    use winreg::enums::*;
    use winreg::RegKey;

    pub fn copier_au_presse_papier(texte: &str) -> Result<(), String> {
        let mut clipboard =
            Clipboard::new().map_err(|e| format!("Erreur initialisation : {}", e))?;

        clipboard
            .set_text(texte.to_string())
            .map_err(|e| format!("Erreur copie : {}", e))?;

        Ok(())
    }

    // Dans utils.rs -> pub mod system
    pub fn sauvegarder_dans_fichier(mdp: &str, type_mdp: &str, label: &str) -> std::io::Result<()> {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut fichier = OpenOptions::new()
            .create(true)
            .append(true)
            .open("historique_mdp.txt")?;

        let ligne_claire = format!("[{:<20}] Type: {:<6} | MDP: {}", label, type_mdp, mdp);
        let key: [u8; 32] = obtenir_ou_creer_cle();

        // CHIFFREMENT
        let cipher = Aes256Gcm::new((&key).into());
        let nonce = Nonce::from_slice(b"unique nonce"); // En vrai, le nonce doit changer à chaque fois
        let ligne_chiffree = cipher
            .encrypt(nonce, ligne_claire.as_bytes())
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Erreur de chiffrement"))?;

        // On stocke en hexadécimal pour que le fichier texte reste "propre"
        let hex_string = hex::encode(ligne_chiffree);
        writeln!(fichier, "{}", hex_string)?;

        Ok(())
    }

    pub fn rechercher_dans_fichier(nom: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let key: [u8; 32] = obtenir_ou_creer_cle();

        let fichier = File::open("historique_mdp.txt")?;
        let lecteur = BufReader::new(fichier);
        let cipher = Aes256Gcm::new((&key).into());
        let nonce = Nonce::from_slice(b"unique nonce");

        println!("--- Recherche sécurisée pour '{}' ---", nom);

        for ligne in lecteur.lines() {
            let l = ligne?;
            // DECHIFFREMENT
            if let Ok(bytes_chiffres) = hex::decode(l.trim()) {
                if let Ok(data_claire) = cipher.decrypt(nonce, bytes_chiffres.as_ref()) {
                    let texte = String::from_utf8_lossy(&data_claire);
                    if texte.to_lowercase().contains(&nom.to_lowercase()) {
                        println!("{}", texte);
                    }
                }
            }
        }
        Ok(())
    }

    // Dans pub mod system { ... }

    pub fn rechercher_dans_fichier_gui(nom: &str) -> std::io::Result<String> {
        use aes_gcm::{
            aead::{Aead, KeyInit},
            Aes256Gcm, Nonce,
        };
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let key = obtenir_ou_creer_cle();
        let cipher = Aes256Gcm::new((&key).into());
        let nonce = Nonce::from_slice(b"unique nonce");

        let fichier = match File::open("historique_mdp.txt") {
            Ok(f) => f,
            Err(_) => return Ok("Erreur : Fichier historique inexistant.".to_string()),
        };

        let lecteur = BufReader::new(fichier);
        let mut resultats_cumules = String::new();
        let mut trouve = false;

        for ligne in lecteur.lines() {
            let l = ligne?;
            let ligne_nettoyee = l.trim(); // Crucial pour hex::decode
            if ligne_nettoyee.is_empty() {
                continue;
            }

            if let Ok(bytes) = hex::decode(ligne_nettoyee) {
                if let Ok(data) = cipher.decrypt(nonce, bytes.as_ref()) {
                    let texte = String::from_utf8_lossy(&data).to_string();
                    // Comparaison insensible à la casse
                    if texte.to_lowercase().contains(&nom.to_lowercase()) {
                        resultats_cumules.push_str(&texte);
                        resultats_cumules.push('\n');
                        trouve = true;
                    }
                }
            }
        }

        if !trouve {
            Ok(format!("Aucun résultat trouvé pour '{}'", nom))
        } else {
            Ok(resultats_cumules)
        }
    }

    pub fn obtenir_ou_creer_cle() -> [u8; 32] {
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        // On crée ou ouvre la clé dans le registre utilisateur
        let (key_reg, _) = hkcu
            .create_subkey("Software\\MonGenerateurMDP")
            .expect("Erreur d'accès au registre");

        // 1. Tentative de lecture
        match key_reg.get_raw_value("SecretKey") {
            Ok(val) => {
                let mut cle = [0u8; 32];
                // On s'assure de ne copier que les 32 premiers octets au cas où
                cle.copy_from_slice(&val.bytes[..32]);
                cle
            }
            Err(_) => {
                // 2. Génération si absente
                let mut nouvelle_cle = [0u8; 32];
                rand::thread_rng().fill_bytes(&mut nouvelle_cle);

                // 3. Sauvegarde (Correction du type .into() ici)
                key_reg
                    .set_raw_value(
                        "SecretKey",
                        &winreg::RegValue {
                            vtype: REG_BINARY,
                            bytes: nouvelle_cle.to_vec().into(), // .into() est crucial ici
                        },
                    )
                    .expect("Erreur de sauvegarde de la clé");

                println!("--- Première utilisation : Clé de sécurité générée dans le registre Windows ---");
                nouvelle_cle
            }
        }
    }
}

pub mod generator {
    use rand::Rng; // Pour générer l'aléatoire

    pub enum TypeMdp {
        Alphabetique,
        Numerique,
        Hexadecimal,
        Mixte, // Le mode actuel (lettres + chiffres + symboles)
    }

    // Cette fonction est pure : elle prend des entrées et retourne un résultat.
    // Elle ne dépend pas de la console, ce qui la rend réutilisable partout.
    pub fn generer_mot_de_passe(longueur: usize, type_mdp: TypeMdp) -> String {
        // On définit le "pool" de caractères selon le choix
        let charset = match type_mdp {
            TypeMdp::Alphabetique => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ",
            TypeMdp::Numerique => "0123456789",
            TypeMdp::Hexadecimal => "0123456789ABCDEF",
            TypeMdp::Mixte => {
                "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()_+-="
            }
        };

        let mut rng = rand::thread_rng();

        (0..longueur)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset.chars().nth(idx).unwrap()
            })
            .collect()
    }
}
