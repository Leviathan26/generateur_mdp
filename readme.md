# 🛡️ Coffre-fort & Générateur de Mots de Passe GesPass

Un utilitaire de sécurité **hybride** ultra-léger conçu en Rust. Il permet de générer des mots de passe robustes, de les copier au presse-papier de manière éphémère et de les stocker dans un historique local **entièrement chiffré**.

![Interface App](https://shields.io)
![Security](https://shields.io)
![Platform](https://shields.io)

## ✨ Fonctionnalités

- **Génération Flexible** : 4 modes (Mixte, Alphabétique, Numérique, Hexadécimal) avec longueur personnalisable.
- **Double Interface** :
  - **GUI** : Une interface moderne et fluide conçue avec **Slint**.
  - **CLI** : Une interface en ligne de commande pour les utilisateurs avancés et l'automatisation.
- **Sécurité Maximale** :
  - Chiffrement des données via l'algorithme **AES-256-GCM**.
  - Clé de chiffrement unique générée au premier lancement et stockée dans le **Registre Windows**.
- **Gestionnaire Intégré** : Sauvegarde avec étiquettes (ex: google.com) et moteur de recherche instantané.
- **Presse-papier Intelligent** : Copie optionnelle pour éviter de laisser des traces en mémoire.

## 🚀 Installation rapide (Windows)

1. Téléchargez le fichier `generateur_mdp.exe` depuis les Releases.
2. Lancez l'application. Au premier démarrage, une clé de sécurité unique sera créée dans votre registre.

## 💻 Utilisation

### Mode Graphique (GUI)

Double-cliquez sur l'exécutable. L'interface vous permet de générer, sauvegarder et rechercher vos mots de passe via des boutons intuitifs. La touche **Entrée** lance automatiquement la recherche ou la sauvegarde.

### Mode Ligne de Commande (CLI)

Ouvrez un terminal et utilisez les arguments suivants :

```bash
# Générer un mot de passe de 24 caractères et le copier
generateur_mdp 24 mixte -c

# Sauvegarder un mot de passe pour un site spécifique
generateur_mdp 16 alpha --save facebook.com

# Rechercher un mot de passe dans l'historique chiffré
generateur_mdp --find google
