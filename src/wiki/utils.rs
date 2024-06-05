// THIS IS THE FILE: src/wiki/utils.rs

pub fn get_snippet(content: &str, snippet_length: usize) -> Result<String, Box<dyn std::error::Error>> {
    let mut snippet = content.lines().take(snippet_length / 2).collect::<Vec<&str>>().join("\n");

    if content.len() > snippet_length {
        snippet.push_str(" ...");
    }

    Ok(snippet)
}
