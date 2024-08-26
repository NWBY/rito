use clap::Parser;
use highlighter::{highlight::SyntaxHighlighter, theme::ThemeManager};
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use syntect::html;

pub mod highlighter;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name",
    about = "Rito Syntax Highlighter"
)]
struct Opts {
    /// The programming language of the input
    #[clap(short, long)]
    language: String,

    /// Input file (use - for stdin)
    #[clap(short, long)]
    input: Option<String>,

    /// Output file (omit for stdout)
    #[clap(short, long)]
    output: Option<PathBuf>,

    /// Theme name
    #[clap(short, long, default_value = "std-light")]
    theme: String,

    /// List available themes
    #[clap(long)]
    list_themes: bool,

    /// Highlight as diff (additions and deletions)
    #[clap(long)]
    diff: bool,

    /// Show line numbers
    #[clap(long)]
    line_numbers: bool,

    /// Path to the styles directory
    #[clap(long, default_value = "styles")]
    styles_path: PathBuf,

    /// Output full HTML file
    #[clap(short, long)]
    full: bool,
}

fn main() -> io::Result<()> {
    let opts: Opts = Opts::parse();
    let highlighter = SyntaxHighlighter::new();
    let theme_manager = ThemeManager::new();

    if opts.list_themes {
        println!("Available themes:");
        for theme in theme_manager.list_themes() {
            println!("- {}", theme);
        }
        return Ok(());
    }

    let theme = if theme_manager.is_valid_theme(&opts.theme) {
        opts.theme
    } else {
        eprintln!("Theme '{}' not found. Using std-light.", opts.theme);
        "std-light".to_string()
    };

    // Read input
    let code = if let Some(input) = opts.input {
        if input == "-" {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer)?;
            buffer
        } else {
            fs::read_to_string(input)?
        }
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    // Generate highlighted HTML
    let highlighted = if opts.diff {
        highlighter.highlight_with_diff(&code, &opts.language, opts.line_numbers)
    } else {
        highlighter.highlight(&code, &opts.language, opts.line_numbers)
    };

    let base_css_path = opts.styles_path.join("base.css");
    let theme_css_path = opts.styles_path.join(format!("{}.css", theme));

    let html_output = if opts.full {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>Rito Syntax Highlighter</title>
    <link rel="stylesheet" href="{}">
    <link rel="stylesheet" href="{}">
</head>
<body>
{}
</body>
</html>"#,
            base_css_path.display(),
            theme_css_path.display(),
            highlighted
        )
    } else {
        format!(
            r#"<div>
    {}
</div>"#,
            highlighted
        )
    };

    // Write output
    if let Some(output_path) = opts.output {
        fs::write(output_path, html_output)?;
    } else {
        io::stdout().write_all(html_output.as_bytes())?;
    }

    Ok(())
}
