use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct SyntaxHighlighter {
    syntax_set: SyntaxSet,
    theme_set: ThemeSet,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        SyntaxHighlighter {
            syntax_set: SyntaxSet::load_defaults_newlines(),
            theme_set: ThemeSet::load_defaults(),
        }
    }

    pub fn highlight(&self, code: &str, language: &str, show_line_numbers: bool) -> String {
        let syntax = self
            .syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"]; // Default theme

        let mut html = String::from("<div class=\"highlight\">");
        let mut line_count = 1;

        for line in LinesWithEndings::from(code) {
            let highlighted = highlighted_html_for_string(line, &self.syntax_set, syntax, theme);

            html.push_str("<div class=\"line\">");

            if show_line_numbers {
                html.push_str(&format!(
                    "<span class=\"line-number\">{}</span>",
                    line_count
                ));
            }

            html.push_str(&format!(
                "<span class=\"code\">{}</span>",
                highlighted.unwrap()
            ));

            html.push_str("</div>");

            line_count += 1;
        }

        html.push_str("</div>");
        html
    }

    pub fn highlight_with_diff(
        &self,
        code: &str,
        language: &str,
        show_line_numbers: bool,
    ) -> String {
        let syntax = self
            .syntax_set
            .find_syntax_by_token(language)
            .unwrap_or_else(|| self.syntax_set.find_syntax_plain_text());

        let theme = &self.theme_set.themes["base16-ocean.dark"]; // Default theme

        let mut html = String::from("<div class=\"highlight diff\">");
        let mut line_count = 1;

        for line in LinesWithEndings::from(code) {
            let highlighted =
                highlighted_html_for_string(line, &self.syntax_set, syntax, theme).unwrap();

            let (class, content) = if line.starts_with('+') {
                ("addition", &highlighted[1..])
            } else if line.starts_with('-') {
                ("deletion", &highlighted[1..])
            } else {
                ("", &highlighted[..])
            };

            html.push_str(&format!("<div class=\"line {}\">", class));

            if show_line_numbers {
                html.push_str(&format!(
                    "<span class=\"line-number\">{}</span>",
                    line_count
                ));
            }

            html.push_str(&format!(
                "<span class=\"diff-symbol\">{}</span>",
                if !class.is_empty() { &line[..1] } else { " " }
            ));

            html.push_str(&format!("<span class=\"code\">{}</span>", content));

            html.push_str("</div>");

            line_count += 1;
        }

        html.push_str("</div>");
        html
    }
}

// ... (keep the tests)
