# SyntectCss

Convert Textmate `.tmtheme` themes to `css` based on `syntect` keywords.

This copy-pastes code from [syntect](https://github.com/trishume/syntect) in order to parse `tmtheme` files.

## Usage

You can generate a css file via:

``` sh
syntectcss -s name-of-theme.tmtheme > output.css
```

This will print `css` to the `stdout`. The css looks like this:

``` css
pre > code * {
	color: #c0c5ce;
	background-color: #2b303b;
}

pre > code .variable.parameter.function {
	color: #c0c5ce;
}

pre > code .comment {
	color: #65737e;
}
```

## Options

These useful commandline options exist

### Prefix

Sometimes the generated classes collide with your existing `css` classes. Setting a prefix helps alleviate this:

``` sh
syntectcss -p "abc-" -s name-of-theme.tmtheme > output.css
```

The `-p "abc-"` parameter will lead to this `css`:

``` css
pre > code .abc-variable.parameter.function {
	color: #c0c5ce;
}

pre > code .abc-comment {
	color: #65737e;
}
```

### Root Selector

If your code is not in the `pre > code` selector, you can change that too:

``` sh
syntectcss -f "div.code > pre.main" -s name-of-theme.tmtheme > output.css
```

Will lead to this `css`:

``` css
div.code > pre.main .abc-variable.parameter.function {
	color: #c0c5ce;
}
```

## With Syntect

In order to generate html from syntect that consumes this css, you can have a look at the [examples/usage.rs](examples/usage.rs) example.

The gist is this code though:

``` rs
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
```

Otherwise, run the example via:

``` sh
cargo run --example usage
```

## Dark Mode / Light Mode

If you want to support dark and light mode you can simply convert two different `.tmtheme` files and wrap
one of them in a 

``` css
@media (prefers-color-scheme: dark) {
    ...
}
```