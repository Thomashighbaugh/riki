// src/wiki/utils.rs

pub fn sanitize_tag(tag: &str) -> String {
    tag.chars()
        .map(|c| match c {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => c, // Allow these characters
            _ => '_', // Replace invalid characters with '_'
        })
        .collect::<String>()
}

fn get_editor_input() -> Result<String, Box<dyn std::error::Error>> {
    let default_editors = vec!["vim", "nano", "code", "emacs", "gedit", "kate"];

    println!("Choose an editor from the list:");
    for (i, editor) in default_editors.iter().enumerate() {
        println!("{}. {}", i + 1, editor);
    }

    loop {
        print!("Enter the number of your choice: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<usize>() {
            Ok(choice) if choice > 0 && choice <= default_editors.len() => {
                return Ok(default_editors[choice - 1].to_string());
            }
            _ => println!("Invalid choice. Please enter a number from the list."),
        }
    }
}

fn is_valid_editor(editor_command: &str) -> bool {
    // Try to find the editor command in the system's PATH
    if let Ok(path) = which::which(editor_command) {
        println!("Editor found at: {}", path.display()); // Optional: Informative message
        true // Editor found in PATH
    } else {
        eprintln!(
            "Warning: Editor '{}' not found in your system's PATH.",
            editor_command
        );
        false // Editor not found
    }
}
