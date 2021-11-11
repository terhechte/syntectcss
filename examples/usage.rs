use syntect::html::ClassedHTMLGenerator;
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

struct ExampleHighlighter<'a> {
    language: &'a str,
    code: &'a str,
    syntax_set: SyntaxSet,
}

impl<'a> ExampleHighlighter<'a> {
    pub fn new(language: &'a str, code: &'a str) -> ExampleHighlighter<'a> {
        let ps = SyntaxSet::load_defaults_newlines();
        ExampleHighlighter {
            language,
            code,
            syntax_set: ps,
        }
    }

    fn highlight(&self) -> String {
        let syntax = match self.syntax_set.find_syntax_by_name(&self.language) {
            Some(s) => s,
            None => match self.syntax_set.find_syntax_by_extension(&self.language) {
                Some(s) => s,
                None => self.syntax_set.find_syntax_plain_text(),
            },
        };

        let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
            &syntax,
            &self.syntax_set,
            syntect::html::ClassStyle::SpacedPrefixed { prefix: "abc-" },
        );
        let lines = LinesWithEndings::from(&self.code);
        for line in lines {
            html_generator.parse_html_for_line_which_includes_newline(&line);
        }
        html_generator.finalize()
    }
}

fn main() {
    let code = r#"
fn example<'a>(content: &'a str) -> String {
    format!("{}", content)
}
    "#;
    let language = "rs";
    let highlighter = ExampleHighlighter::new(&language, &code);
    println!("{}", highlighter.highlight());
}
