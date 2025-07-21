### **Ce que j’ai appris avec ce cours d'introduction a Rust**

1. **Struct et implémentation de méthodes**

   - Création de structures (`struct CompteBancaire`) pour regrouper des données.
   - Utilisation de `impl` pour définir des **méthodes associées** (comme `new`, `afficher_solde`, `retrait`, `depot`).
   - Utilisation de `&self` (lecture seule) et `&mut self` (modification).

2. **Gestion de collections avec HashMap ou Vec**

   - Dans mon projet, j’ai utilisé `HashMap` pour stocker plusieurs comptes bancaires.
   - Utilisation de `Vec` pour des listes simples (non utilisé ici, mais utile à connaître).
   - Utilisation de `std::collections::HashMap` pour stocker plusieurs comptes bancaires.
   - Insertion avec `.insert()` et accès avec `.get_mut()`.

3. **Entrées / sorties (I/O)**

   - Lecture d’entrées utilisateur avec `io::stdin().read_line(&mut input)`.
   - Écriture dans la console avec `println!` et `print!`.
   - Utilisation de `io::stdout().flush()` pour forcer l’affichage.

4. **Gestion d’erreurs simples**

   - Vérification des entrées (montant > 0).
   - Utilisation de `match` et `parse::<f32>()` pour convertir des chaînes en nombres avec gestion d’erreur.

5. **Boucles et logique de menu**

   - Boucle `loop` pour afficher un menu interactif.
   - Utilisation de `match` pour gérer les choix de l’utilisateur.

6. **Ownership et emprunt**

   - Compréhension de l’**emprunt mutable** (`&mut self`, `&mut CompteBancaire`) pour pouvoir modifier le compte sans transférer la propriété.

7. **Formatage des chaînes**

   - Formatage avancé avec `println!("Solde: {:.2}€", self.solde)` pour afficher avec 2 décimales.

---

> Ce projet m’a permis de comprendre les bases de Rust : la définition de structures, l’utilisation des collections comme HashMap, la gestion des entrées/sorties, la boucle principale d’un programme interactif et les notions fondamentales d’ownership et d’emprunt.
