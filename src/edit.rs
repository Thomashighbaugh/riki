// edit.rs
use std::path::Path;
use std::process::Command;

pub fn edit_page(args: &[String]) {
    let wiki_dir = &args[0];
    let page_name = &args[1];

    let page_path = Path::new(wiki_dir).join(format!("{}.md", page_name));

    // Open page for editing
    let editor = match std::env::var("EDITOR") {
        Ok(editor) => editor,
        Err(_) => {
            println!("No EDITOR environment variable set, defaulting to 'nano'.");
            "nano".to_string()
        }
    };

    // Spawn the editor process
    let mut command = Command::new(&editor);
    command.arg(&page_path);
    match command.spawn() {
        Ok(mut child) => {
            if let Err(e) = child.wait() {
                println!("Error: Failed to wait for editor process: {}", e);
            }
        },
        Err(e) => {
            println!("Error: Failed to spawn editor process: {}", e);
        }
    }
}
