use serde::{Deserialize, Serialize}; // Import necessary traits for serialization and deserialization
use std::fs::File; // File handling
use std::io::Read; // Reading from file

/// Represents a paragraph in an article.
///
/// Each paragraph has a `name`, which is the content or title of that paragraph.
#[derive(Serialize, Deserialize)]
struct Paragraph {
    /// The text or content of the paragraph.
    name: String,
}

/// Represents an article with multiple paragraphs.
///
/// It contains the `article` title, the `author`, and a list of `paragraph`s.
#[derive(Serialize, Deserialize)]
struct Article {
    /// The title of the article.
    article: String,

    /// The name of the article's author.
    author: String,

    /// A list of paragraphs in the article.
    paragraph: Vec<Paragraph>,
}

fn main() {
    // Call the function to read the JSON file and parse it into an `Article` struct.
    let parsed: Article = read_json_from_file("article.json");

    // Print the name of the first paragraph
    println!(
        "The name of the first paragraph is: {}",
        parsed.paragraph[0].name
    );
}

/// Reads a JSON file from the specified file path and parses it into an `Article` struct.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the name of the file to be read.
///
/// # Returns
///
/// This function returns an `Article` struct parsed from the JSON file.
///
/// # Panics
///
/// This function will panic if:
/// * The file does not exist.
/// * There is an error reading the file.
/// * The file's contents are not valid JSON according to the `Article` structure.
///
/// # Example
///
/// ```
/// let article = read_json_from_file("article.json");
/// println!("Article title: {}", article.article);
/// ```
fn read_json_from_file(file_path: &str) -> Article {
    // Open the file located at `file_path`.
    let mut file = File::open(file_path).expect("File not found");

    // Create a string to store the contents of the file.
    let mut content = String::new();

    // Read the file's contents into the string.
    file.read_to_string(&mut content)
        .expect("Error reading the file");

    // Parse the JSON string into the `Article` struct using Serde.
    let parsed: Article = serde_json::from_str(&content).unwrap();

    // Return the parsed `Article`.
    parsed
}
