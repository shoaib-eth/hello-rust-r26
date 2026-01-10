# 🦀 Project 1 – Build a CLI Tool with Rust (Markdown → HTML)

## Project Overview
This project is a **command-line tool written in Rust** that:

1. Takes a **Markdown (.md)** file as input
2. Converts it into **HTML**
3. Wraps the HTML inside a proper HTML page
4. Saves the result as an **.html file**

This is a **real-world CLI project** combining:
- Argument parsing (`clap`)
- File handling (`std::fs`)
- Markdown parsing (`pulldown_cmark`)
- HTML templating (`maud`)

---

## CLI Command Used

```bash
cargo run -- -i Rust_Intro.md -o Rust_Intro.html
```

### Meaning of This Command

- `cargo run` → builds and runs the Rust project
- `--` → separates Cargo arguments from **your program's arguments**
- `-i Rust_Intro.md` → input Markdown file
- `-o Rust_Intro.html` → output HTML file

Without `--`, Cargo would try to parse `-i` and `-o` itself.

---

## Project Dependencies (Conceptual)

This project uses these crates:

- `clap` → Parse CLI arguments
- `pulldown_cmark` → Convert Markdown to raw HTML
- `maud` → Wrap HTML content into a full HTML document

Each crate has **one clear responsibility**.

---

## Full Code (Reference)

```rust
use clap::Parser;
use maud::{DOCTYPE, Markup, html};
use pulldown_cmark::{Options, Parser as MarkdownParser, html};
use std::{fs, path::PathBuf};
```

---

## Step-by-Step Deep Explanation

---

## 1. Importing Required Modules

```rust
use clap::Parser;
```

- Imports the `Parser` derive macro
- Enables automatic CLI argument parsing

```rust
use maud::{DOCTYPE, Markup, html};
```

- `DOCTYPE` → adds `<!DOCTYPE html>`
- `Markup` → represents HTML output safely
- `html!` → macro to write HTML in Rust syntax

```rust
use pulldown_cmark::{Options, Parser as MarkdownParser, html};
```

- `Options` → enable Markdown features
- `MarkdownParser` → parses Markdown text
- `html::push_html` → converts Markdown into HTML string

```rust
use std::{fs, path::PathBuf};
```

- `fs` → file read/write
- `PathBuf` → safe file path handling

---

## 2. Defining CLI Arguments Using `clap`

```rust
#[derive(Parser, Debug)]
struct Args {
    /// Input Markdown File Path
    #[arg(long, short)]
    input: PathBuf,

    /// Output HTML File Path
    #[arg(long, short)]
    output: Option<PathBuf>,
}
```

### What This Means

- `Args` struct represents **CLI input**
- Each field = one command-line argument

### Input Argument

```bash
-i file.md
--input file.md
```

- Required argument
- Stored as `PathBuf`

### Output Argument

```bash
-o file.html
--output file.html
```

- Optional argument
- Wrapped in `Option<PathBuf>`

This means:
- Output path may or may not be provided

---

## 3. Rendering a Full HTML Page

```rust
fn render_html_page(content: &str) -> Markup {
```

- Takes raw HTML content
- Returns structured HTML (`Markup`)

---

### Inside `render_html_page`

```rust
html! {
    (DOCTYPE)
    html {
        head {
            meta charset = "utf-8";
            title { "Markdown To HTML Output" }
        }
        body {
            (maud::PreEscaped(content.to_string()))
        }
    }
}
```

Explanation like a child-level breakdown:

- `(DOCTYPE)` → tells browser this is HTML5
- `html {}` → root tag
- `head {}` → metadata
- `body {}` → visible content

### Why `PreEscaped`?

- Markdown parser already gives valid HTML
- We **do not want Maud to escape it**
- `PreEscaped` tells Maud: "trust this HTML"

---

## 4. Program Entry Point – `main()`

```rust
let args = Args::parse();
```

- Reads CLI input
- Converts flags into `Args` struct

Example result:
```text
Args {
  input: "Rust_Intro.md",
  output: Some("Rust_Intro.html")
}
```

---

## 5. Reading the Markdown File

```rust
let markdown_input = fs::read_to_string(&args.input)
    .expect("Failed To Read To String");
```

- Reads file contents into a `String`
- Program crashes with message if file is missing

---

## 6. Markdown Parser Configuration

```rust
let mut options = Options::empty();
options.insert(Options::ENABLE_STRIKETHROUGH);
```

- Creates Markdown configuration
- Enables `~~strikethrough~~` support

---

## 7. Parsing Markdown → HTML

```rust
let parser = MarkdownParser::new_ext(&markdown_input, options);
```

- Converts Markdown text into tokens

```rust
let mut html_output = String::new();
html::push_html(&mut html_output, parser);
```

- Pushes generated HTML into a string

Now `html_output` contains **raw HTML body content**.

---

## 8. Wrapping HTML in a Full Page

```rust
let full_html_output = render_html_page(&html_output).into_string();
```

- Adds `<html>`, `<head>`, `<body>`
- Converts `Markup` → `String`

---

## 9. Writing Output File

```rust
match &args.output {
    Some(path) => fs::write(path, full_html_output)
        .expect("Failed To Write!"),
    None => println!("Path Not Provided"),
}
```

### Behavior

- If `-o` is provided → write HTML file
- If not provided → show message

This avoids crashing when output path is missing.

---

## `--help` Command

```bash
cargo run -- --help
```

### What Happens

- `clap` auto-generates help text
- Shows usage, flags, and descriptions

Example output (simplified):
```text
Usage: app -i <INPUT> [-o <OUTPUT>]

Options:
  -i, --input <INPUT>    Input Markdown File Path
  -o, --output <OUTPUT>  Output HTML File Path
  -h, --help             Print help information
```

---

## Complete Flow (Mental Model)

1. User runs CLI command
2. `clap` parses arguments
3. Markdown file is read
4. Markdown → HTML conversion happens
5. HTML is wrapped into a page
6. Output file is written

---

## Why This Project Is Important

This project teaches:
- Real CLI tooling
- File handling
- External crates usage
- Clean Rust architecture

This is **industry-grade learning**, not toy code.

---

## Summary

- This is a Markdown to HTML CLI tool
- Uses `clap`, `pulldown_cmark`, and `maud`
- Clean separation of responsibilities
- Safe, readable, and extensible design

---

## Interview Tip
"I built a Rust CLI tool that parses Markdown, converts it to HTML, and uses clap for argument parsing with strong type safety."

---

**End of Notes**

