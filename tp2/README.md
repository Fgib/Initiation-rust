# TP2 - Gestionnaire de Fichiers Rust

## Description

Ce programme est un gestionnaire de fichiers complet écrit en Rust qui permet de :

- Lire des fichiers
- Écrire dans des fichiers (nouveau contenu ou ajout)
- Modifier des fichiers (ligne par ligne ou contenu complet)
- Supprimer définitivement des fichiers
- Naviguer dans les répertoires
- Afficher les informations des fichiers avec horodatage

## Fonctionnalités Implémentées

### Concepts Rust Utilisés

1. **Ownership et Borrowing** :

   - Gestion de la mémoire avec les références `&str` et `&self`
   - Transfert de propriété avec `String` et `Vec<String>`
   - Borrowing dans les méthodes de la structure

2. **Pattern Matching avec `match`** :

   - Menu principal avec correspondance des choix utilisateur
   - Gestion des erreurs avec `Result<T, E>`
   - Correspondance pour les confirmations utilisateur

3. **Loops et While** :

   - Boucle principale `loop` pour le menu interactif
   - Boucles `while` pour la lecture continue de l'entrée utilisateur
   - Itération sur les fichiers et les lignes

4. **Structure avec `impl`** :

   - Structure `FileManager` avec état (répertoire courant)
   - Implémentation de méthodes avec `impl FileManager`
   - Méthodes pour toutes les opérations sur fichiers

5. **Chrono pour les dates** :
   - Affichage des dates de création et modification
   - Horodatage des opérations
   - Formatage des dates en français

## Utilisation

### Compilation et Exécution

```bash
cd tp2
cargo build
cargo run
```

### Options du Menu

1. **Lister les fichiers** - Affiche tous les fichiers du répertoire courant avec leurs informations
2. **Lire un fichier** - Lit et affiche le contenu d'un fichier
3. **Écrire dans un fichier** - Crée un nouveau fichier ou écrase/ajoute du contenu
4. **Modifier un fichier** - Modification interactive :
   - Remplacement complet du contenu
   - Ajout de contenu
   - Modification ligne par ligne
5. **Supprimer un fichier** - Suppression définitive avec confirmation
6. **Changer de répertoire** - Navigation dans l'arborescence
7. **Informations sur un fichier** - Métadonnées détaillées
8. **Quitter** - Ferme le programme

### Exemples d'Utilisation

#### Créer et écrire un fichier

```
Choisir option 3
Nom: test.txt
Contenu: Hello World!
FIN
```

#### Modifier ligne par ligne

```
Choisir option 4
Nom: test.txt
Choisir option 3 (modification ligne par ligne)
Ajouter, modifier ou supprimer des lignes
Sauvegarder avec option 4
```

## Architecture du Code

### Structure Principal

```rust
struct FileManager {
    current_directory: String,
}
```

### Méthodes Implémentées

- `new()` - Constructeur
- `list_files()` - Liste les fichiers avec métadonnées
- `read_file()` - Lecture de fichiers
- `write_file()` - Écriture (nouveau/ajout)
- `modify_file()` - Modification interactive
- `modify_line_by_line()` - Édition ligne par ligne
- `delete_file()` - Suppression avec confirmation
- `change_directory()` - Navigation
- `get_file_info()` - Informations détaillées

### Gestion des Erreurs

- Utilisation de `Result<T, E>` pour toutes les opérations
- Messages d'erreur explicites
- Gestion des cas d'échec (fichier inexistant, permissions, etc.)

### Dépendances

- `std::fs` - Opérations sur le système de fichiers
- `std::io` - Entrées/sorties
- `chrono` - Gestion des dates et heures

## Concepts Avancés Utilisés

1. **Error Handling** avec `Box<dyn std::error::Error>`
2. **File I/O** avec `BufReader` pour la lecture efficace
3. **Path manipulation** avec `std::path::Path`
4. **Metadata** pour les informations système
5. **DateTime** avec conversion automatique des timestamps système
