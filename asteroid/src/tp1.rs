use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct CompteBancaire {
    numero: String,
    titulaire: String,
    solde: f64,
}

impl CompteBancaire {
    fn new(numero: String, titulaire: String, solde_initial: f64) -> Self {
        CompteBancaire {
            numero,
            titulaire,
            solde: solde_initial,
        }
    }

    fn afficher_solde(&self) {
        println!(
            "Compte: {} | Titulaire: {} | Solde: {:.2}€",
            self.numero, self.titulaire, self.solde
        );
    }

    fn retrait(&mut self, montant: f64) {
        if montant <= 0.0 {
            println!("Le montant doit être positif");
            return;
        }

        if montant > self.solde {
            println!("Solde insuffisant");
            return;
        }

        self.solde = self.solde - montant;
        println!(
            "Retrait de {:.2}€ effectué. Nouveau solde: {:.2}€",
            montant, self.solde
        );
    }

    fn depot(&mut self, montant: f64) {
        if montant <= 0.0 {
            println!("Le montant doit être positif");
            return;
        }

        self.solde = self.solde + montant;
        println!(
            "Dépôt de {:.2}€ effectué. Nouveau solde: {:.2}€",
            montant, self.solde
        );
    }
}

struct GestionnaireComptes {
    comptes: HashMap<String, CompteBancaire>,
}

impl GestionnaireComptes {
    fn new() -> Self {
        GestionnaireComptes {
            comptes: HashMap::new(),
        }
    }

    fn ajouter_compte(&mut self, compte: CompteBancaire) {
        self.comptes.insert(compte.numero.clone(), compte);
    }

    fn lister_comptes(&self) {
        if self.comptes.is_empty() {
            println!("Aucun compte enregistré.");
            return;
        }

        println!("\n=== Liste des comptes ===");
        for compte in self.comptes.values() {
            compte.afficher_solde();
        }
        println!("========================");
    }

    fn selectionner_compte(&mut self) -> Option<&mut CompteBancaire> {
        if self.comptes.is_empty() {
            println!("Aucun compte disponible.");
            return None;
        }

        println!("\nComptes disponibles:");
        let mut i = 1;
        for numero in self.comptes.keys() {
            println!("{}. {}", i, numero);
            i = i + 1;
        }

        print!("Sélectionnez un compte (numéro): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let numero = input.trim();
        if self.comptes.contains_key(numero) {
            return self.comptes.get_mut(numero);
        } else {
            println!("Compte non trouvé.");
            return None;
        }
    }
}

fn lire_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn lire_montant(prompt: &str) -> f64 {
    loop {
        let input = lire_input(prompt);
        match input.parse::<f64>() {
            Ok(montant) => return montant,
            Err(_) => println!("Montant invalide. Essayez encore."),
        }
    }
}

fn afficher_menu() {
    let options = [
        "Afficher solde",
        "Retrait",
        "Dépôt",
        "Liste comptes",
        "Quitter",
    ];

    println!("\n=== Menu Principal ===");
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }
    println!("======================");
}

fn creer_comptes_demo() -> GestionnaireComptes {
    let mut gestionnaire = GestionnaireComptes::new();

    let compte1 = CompteBancaire::new("12345".to_string(), "Alice Dupont".to_string(), 1500.0);
    let compte2 = CompteBancaire::new("67890".to_string(), "Bob Martin".to_string(), 2300.0);
    let compte3 = CompteBancaire::new("11111".to_string(), "Claire Bernard".to_string(), 800.0);

    gestionnaire.ajouter_compte(compte1);
    gestionnaire.ajouter_compte(compte2);
    gestionnaire.ajouter_compte(compte3);

    gestionnaire
}

fn main() {
    println!("=== Gestionnaire de Comptes Bancaires ===\n");

    let mut gestionnaire = creer_comptes_demo();

    println!("Voulez-vous créer un nouveau compte ? (o/n)");
    let reponse = lire_input("");
    if reponse.to_lowercase() == "o" || reponse.to_lowercase() == "oui" {
        let numero = lire_input("Numéro de compte: ");
        let titulaire = lire_input("Nom du titulaire: ");
        let solde_initial = lire_montant("Solde initial: ");

        let nouveau_compte = CompteBancaire::new(numero, titulaire, solde_initial);
        gestionnaire.ajouter_compte(nouveau_compte);
        println!("Compte créé avec succès!");
    }

    loop {
        afficher_menu();

        let choix = lire_input("Votre choix: ");

        match choix.as_str() {
            "1" => {
                if let Some(compte) = gestionnaire.selectionner_compte() {
                    println!("\n=== Solde du compte ===");
                    compte.afficher_solde();
                }
            }
            "2" => {
                if let Some(compte) = gestionnaire.selectionner_compte() {
                    let montant = lire_montant("Montant à retirer: ");
                    compte.retrait(montant);
                }
            }
            "3" => {
                if let Some(compte) = gestionnaire.selectionner_compte() {
                    let montant = lire_montant("Montant à déposer: ");
                    compte.depot(montant);
                }
            }
            "4" => {
                gestionnaire.lister_comptes();
            }
            "5" => {
                println!("Au revoir!");
                break;
            }
            _ => {
                println!("Choix invalide. Essayez encore.");
            }
        }

        println!("\nAppuyez sur Entrée pour continuer...");
        let mut pause = String::new();
        io::stdin().read_line(&mut pause).unwrap();
    }
}
