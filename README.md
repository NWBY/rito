# rito

A syntax highlighter written in Rust. Pronounced like "rito" in burrito or Dorito.

## Features

- Supports multiple languages
- Supports inline code highlighting
- Supports line numbers
- Supports diff highlighting
- Supports custom themes
- Supports custom styles

## Usage

### Command-line

```bash
$ rito --help
Rito Syntax Highlighter
```

Passing a file as input and printing to stdout:

```bash
$ rito --language=rust --input=src/main.rs
```

Passing a string as input and printing to stdout:

```bash
$ rito --language=rust --code='fn main() { println!("Hello, world!"); }'
```

Passing a file as input and saving to a file:

```bash
$ rito --language=rust --input=src/main.rs --output=output.html
```

Passing a string as input and saving to a file:

```bash
$ rito --language rust --code "fn main() { println!(\"Hello, world!\"); }" --output highlighted.html
```

Reading from stdin:

```bash
echo "console.log('Hello, world!');" | rito --language javascript
```
