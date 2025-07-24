use chrono::{DateTime, Local};
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::Path;

#[derive(Debug)]
struct FileManager {
    current_directory: String,
}

impl FileManager {
    fn new() -> Self {
        FileManager {
            current_directory: std::env::current_dir()
                .unwrap()
                .to_string_lossy()
                .to_string(),
        }
    }

    fn list_files(&self) -> io::Result<Vec<String>> {
        let mut files = Vec::new();
        let entries = fs::read_dir(&self.current_directory)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let metadata = entry.metadata()?;
            let modified: DateTime<Local> = metadata.modified()?.into();

            let file_info = format!(
                "{} - {} - {} bytes - Modified: {}",
                path.file_name().unwrap().to_string_lossy(),
                if metadata.is_dir() { "DIR" } else { "FILE" },
                metadata.len(),
                modified.format("%Y-%m-%d %H:%M:%S")
            );
            files.push(file_info);
        }
        Ok(files)
    }

    fn read_file(&self, filename: &str) -> Result<String, Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.current_directory).join(filename);

        if !file_path.exists() {
            return Err(format!("File '{}' does not exist", filename).into());
        }

        let mut file = File::open(&file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        println!(
            "File '{}' read at: {}",
            filename,
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );
        Ok(contents)
    }

    fn write_file(
        &self,
        filename: &str,
        content: &str,
        append: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.current_directory).join(filename);

        let mut file = if append {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(&file_path)?
        } else {
            File::create(&file_path)?
        };

        file.write_all(content.as_bytes())?;

        let action = if append { "appended to" } else { "written to" };
        println!(
            "Content {} file '{}' at: {}",
            action,
            filename,
            Local::now().format("%Y-%m-%d %H:%M:%S")
        );
        Ok(())
    }

    fn modify_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.current_directory).join(filename);

        if !file_path.exists() {
            return Err(format!("File '{}' does not exist", filename).into());
        }

        // Read current content
        let current_content = self.read_file(filename)?;
        println!("\nCurrent content:");
        println!("{}", current_content);

        println!("\nChoose modification option:");
        println!("1. Replace entire content");
        println!("2. Append to content");
        println!("3. Replace line by line");

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        match choice {
            "1" => {
                println!("Enter new content (press Ctrl+D when finished):");
                let mut new_content = String::new();
                io::stdin().read_to_string(&mut new_content)?;
                self.write_file(filename, &new_content, false)?;
            }
            "2" => {
                println!("Enter content to append:");
                let mut additional_content = String::new();
                io::stdin().read_line(&mut additional_content)?;
                self.write_file(filename, &additional_content, true)?;
            }
            "3" => {
                self.modify_line_by_line(filename)?;
            }
            _ => println!("Invalid choice"),
        }

        Ok(())
    }

    fn modify_line_by_line(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.current_directory).join(filename);
        let file = File::open(&file_path)?;
        let reader = BufReader::new(file);
        let mut lines: Vec<String> = reader.lines().collect::<Result<Vec<_>, _>>()?;

        loop {
            println!("\nCurrent lines:");
            for (i, line) in lines.iter().enumerate() {
                println!("{}: {}", i + 1, line);
            }

            println!("\nOptions:");
            println!("1. Modify a line");
            println!("2. Delete a line");
            println!("3. Add a line");
            println!("4. Save and exit");

            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let choice = input.trim();

            match choice {
                "1" => {
                    println!("Enter line number to modify:");
                    let mut line_input = String::new();
                    io::stdin().read_line(&mut line_input)?;

                    if let Ok(line_num) = line_input.trim().parse::<usize>() {
                        if line_num > 0 && line_num <= lines.len() {
                            println!("Current line: {}", lines[line_num - 1]);
                            println!("Enter new content:");
                            let mut new_line = String::new();
                            io::stdin().read_line(&mut new_line)?;
                            lines[line_num - 1] = new_line.trim_end().to_string();
                        } else {
                            println!("Invalid line number");
                        }
                    }
                }
                "2" => {
                    println!("Enter line number to delete:");
                    let mut line_input = String::new();
                    io::stdin().read_line(&mut line_input)?;

                    if let Ok(line_num) = line_input.trim().parse::<usize>() {
                        if line_num > 0 && line_num <= lines.len() {
                            lines.remove(line_num - 1);
                            println!("Line {} deleted", line_num);
                        } else {
                            println!("Invalid line number");
                        }
                    }
                }
                "3" => {
                    println!("Enter new line content:");
                    let mut new_line = String::new();
                    io::stdin().read_line(&mut new_line)?;
                    lines.push(new_line.trim_end().to_string());
                }
                "4" => break,
                _ => println!("Invalid choice"),
            }
        }

        let content = lines.join("\n");
        self.write_file(filename, &content, false)?;
        Ok(())
    }

    fn delete_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.current_directory).join(filename);

        if !file_path.exists() {
            return Err(format!("File '{}' does not exist", filename).into());
        }

        // Confirmation prompt
        println!(
            "Are you sure you want to permanently delete '{}'? (yes/no)",
            filename
        );
        let mut confirmation = String::new();
        io::stdin().read_line(&mut confirmation)?;

        match confirmation.trim().to_lowercase().as_str() {
            "yes" | "y" => {
                fs::remove_file(&file_path)?;
                println!(
                    "File '{}' deleted permanently at: {}",
                    filename,
                    Local::now().format("%Y-%m-%d %H:%M:%S")
                );
            }
            _ => {
                println!("Deletion cancelled");
            }
        }

        Ok(())
    }

    fn change_directory(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let new_path = if path.starts_with('/') {
            path.to_string()
        } else {
            Path::new(&self.current_directory)
                .join(path)
                .to_string_lossy()
                .to_string()
        };

        if Path::new(&new_path).exists() {
            self.current_directory = new_path;
            println!("Changed directory to: {}", self.current_directory);
        } else {
            return Err(format!("Directory '{}' does not exist", path).into());
        }

        Ok(())
    }

    fn get_file_info(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file_path = Path::new(&self.current_directory).join(filename);

        if !file_path.exists() {
            return Err(format!("File '{}' does not exist", filename).into());
        }

        let metadata = fs::metadata(&file_path)?;
        let modified: DateTime<Local> = metadata.modified()?.into();
        let created: DateTime<Local> = metadata.created()?.into();

        println!("File Information for '{}':", filename);
        println!("  Size: {} bytes", metadata.len());
        println!(
            "  Type: {}",
            if metadata.is_dir() {
                "Directory"
            } else {
                "File"
            }
        );
        println!("  Created: {}", created.format("%Y-%m-%d %H:%M:%S"));
        println!("  Modified: {}", modified.format("%Y-%m-%d %H:%M:%S"));
        println!("  Permissions: {:?}", metadata.permissions());

        Ok(())
    }
}

fn main() {
    let mut file_manager = FileManager::new();

    println!("=== Gestionnaire de Fichiers Rust ===");
    println!("Démarré le: {}", Local::now().format("%Y-%m-%d %H:%M:%S"));

    loop {
        println!("\n=== Menu Principal ===");
        println!("Répertoire courant: {}", file_manager.current_directory);
        println!("1. Lister les fichiers");
        println!("2. Lire un fichier");
        println!("3. Écrire dans un fichier");
        println!("4. Modifier un fichier");
        println!("5. Supprimer un fichier");
        println!("6. Changer de répertoire");
        println!("7. Informations sur un fichier");
        println!("8. Quitter");
        println!("Choisissez une option (1-8): ");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let choice = input.trim();

                match choice {
                    "1" => match file_manager.list_files() {
                        Ok(files) => {
                            if files.is_empty() {
                                println!("Aucun fichier trouvé dans le répertoire");
                            } else {
                                println!("\nFichiers dans le répertoire:");
                                for file in files {
                                    println!("  {}", file);
                                }
                            }
                        }
                        Err(e) => println!("Erreur lors de la lecture du répertoire: {}", e),
                    },
                    "2" => {
                        println!("Nom du fichier à lire: ");
                        let mut filename = String::new();
                        if let Ok(_) = io::stdin().read_line(&mut filename) {
                            let filename = filename.trim();
                            match file_manager.read_file(filename) {
                                Ok(content) => {
                                    println!("\n=== Contenu du fichier '{}' ===", filename);
                                    println!("{}", content);
                                    println!("=== Fin du fichier ===");
                                }
                                Err(e) => println!("Erreur: {}", e),
                            }
                        }
                    }
                    "3" => {
                        println!("Nom du fichier: ");
                        let mut filename = String::new();
                        if let Ok(_) = io::stdin().read_line(&mut filename) {
                            let filename = filename.trim();

                            println!("Contenu à écrire (tapez 'FIN' sur une ligne séparée pour terminer):");
                            let mut content = String::new();
                            let mut line = String::new();

                            while io::stdin().read_line(&mut line).is_ok() {
                                if line.trim() == "FIN" {
                                    break;
                                }
                                content.push_str(&line);
                                line.clear();
                            }

                            println!("Écraser le fichier existant? (o/n): ");
                            let mut overwrite = String::new();
                            if let Ok(_) = io::stdin().read_line(&mut overwrite) {
                                let append = overwrite.trim().to_lowercase() != "o";

                                match file_manager.write_file(filename, &content, append) {
                                    Ok(_) => println!("Fichier écrit avec succès"),
                                    Err(e) => println!("Erreur: {}", e),
                                }
                            }
                        }
                    }
                    "4" => {
                        println!("Nom du fichier à modifier: ");
                        let mut filename = String::new();
                        if let Ok(_) = io::stdin().read_line(&mut filename) {
                            let filename = filename.trim();
                            match file_manager.modify_file(filename) {
                                Ok(_) => println!("Fichier modifié avec succès"),
                                Err(e) => println!("Erreur: {}", e),
                            }
                        }
                    }
                    "5" => {
                        println!("Nom du fichier à supprimer: ");
                        let mut filename = String::new();
                        if let Ok(_) = io::stdin().read_line(&mut filename) {
                            let filename = filename.trim();
                            match file_manager.delete_file(filename) {
                                Ok(_) => {}
                                Err(e) => println!("Erreur: {}", e),
                            }
                        }
                    }
                    "6" => {
                        println!("Nouveau répertoire: ");
                        let mut path = String::new();
                        if let Ok(_) = io::stdin().read_line(&mut path) {
                            let path = path.trim();
                            match file_manager.change_directory(path) {
                                Ok(_) => {}
                                Err(e) => println!("Erreur: {}", e),
                            }
                        }
                    }
                    "7" => {
                        println!("Nom du fichier: ");
                        let mut filename = String::new();
                        if let Ok(_) = io::stdin().read_line(&mut filename) {
                            let filename = filename.trim();
                            match file_manager.get_file_info(filename) {
                                Ok(_) => {}
                                Err(e) => println!("Erreur: {}", e),
                            }
                        }
                    }
                    "8" => {
                        println!(
                            "Au revoir! Programme terminé le: {}",
                            Local::now().format("%Y-%m-%d %H:%M:%S")
                        );
                        break;
                    }
                    _ => {
                        println!("Option invalide. Veuillez choisir entre 1 et 8.");
                        continue;
                    }
                }
            }
            Err(e) => {
                println!("Erreur lors de la lecture de l'entrée: {}", e);
                continue;
            }
        }
    }
}
