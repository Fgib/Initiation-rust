// TP2 - Gestionnaire de Fichiers Rust
// Concepts démontrés:
// - Ownership & Borrowing
// - Pattern Matching (match)
// - Loops (loop, while)
// - Struct avec impl
// - Chrono pour les dates
// - Gestion d'erreurs avec Result<T, E>

/*
UTILISATION DES CONCEPTS RUST:

1. OWNERSHIP & BORROWING:
   - &self dans les méthodes (borrowing immutable)
   - &mut self pour changer l'état (borrowing mutable)
   - String vs &str (owned vs borrowed strings)

2. PATTERN MATCHING:
   - match sur les choix du menu
   - match sur Result<T, E> pour la gestion d'erreurs
   - match sur les confirmations utilisateur

3. LOOPS:
   - loop principal pour le menu interactif
   - while pour la lecture continue
   - for pour l'itération sur les collections

4. STRUCT + IMPL:
   - FileManager stocke l'état (current_directory)
   - Toutes les méthodes dans impl FileManager
   - Encapsulation des fonctionnalités

5. CHRONO:
   - DateTime<Local> pour les timestamps
   - Formatage des dates
   - Conversion des métadonnées système
*/
