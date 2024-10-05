# Rust JSON Reader Example

This is a simple Rust project that demonstrates how to read and parse JSON files using the [Serde](https://serde.rs/) library. The project includes:

- A struct-based approach for deserializing JSON content.
- Reading JSON data from an external file (`article.json`).
- Example output of parsing and accessing JSON data.

## Project Structure

The main components of this project are:

- **`main.rs`**: The main Rust file that reads the JSON file and prints out the content.
- **`article.json`**: A JSON file that contains sample data representing an article with multiple paragraphs.
- **`Cargo.toml`**: The Rust package configuration file where dependencies like `serde` and `serde_json` are defined.

## JSON File Format

The project uses a simple `article.json` file with the following structure:

```json
{
    "article": "how to work with json in rust",
    "author": "kani",
    "paragraph": [
        {
            "name": "string sentence"
        },
        {
            "name": "body of the paragraph"
        },
        {
            "name": "end of the paragraph"
        }
    ]
}
