# Browser Engine

A toy browser rendering engine written in Rust that parses HTML and CSS, applies styling, performs layout calculations, and renders the result to a PNG image.

This project is based on the excellent tutorial series ["Let's build a browser engine!"](https://limpet.net/mbrubeck/2014/11/05/toy-layout-engine-7-painting.html) by Matt Brubeck.

## Overview

This browser engine implements a simplified rendering pipeline that transforms HTML and CSS into a visual representation. The engine follows the standard browser rendering pipeline:

1. **HTML Parsing** - Parses HTML into a DOM tree
2. **CSS Parsing** - Parses CSS rules and declarations
3. **Style** - Matches CSS rules to DOM nodes and builds a style tree
4. **Layout** - Calculates the position and size of each element (block layout)
5. **Painting** - Rasterizes the layout tree into pixels and outputs a PNG image

## Features

### Currently Implemented

- ✅ HTML parsing into DOM tree
- ✅ CSS parsing (selectors, declarations, values)
- ✅ Style tree construction with rule matching
- ✅ Block-level layout algorithm
- ✅ Basic painting/rasterization
- ✅ Background color rendering
- ✅ Border rendering
- ✅ Padding support
- ✅ Nested block elements
- ✅ PNG output

## Architecture

The project is organized into several modules:

- **`dom.rs`** - DOM node structure and tree representation
- **`html.rs`** - HTML parser
- **`css.rs`** - CSS parser and value types
- **`style.rs`** - Style tree construction and CSS rule matching
- **`layout.rs`** - Block layout algorithm and box model calculations
- **`painting.rs`** - Display list generation and rasterization

## Usage

### Prerequisites

- Rust (latest stable version recommended)
- Cargo

### Building

```bash
cargo build
```

### Running

The program accepts command-line arguments for HTML and CSS files:

```bash
cargo run -- -html <html_file> -css <css_file> -output <output_file>
```

**Default behavior** (if no arguments provided):

- HTML: `./test.html`
- CSS: `./test.css`
- Output: `./output.png`

**Example:**

```bash
cargo run -- --html test.html --css test.css --output result.png
```

### Example Input

**test.html:**

```html
<html lang="en">
  <head>
    <title>Document</title>
  </head>
  <body>
    <div class="a">
      <div class="b">
        <div class="c">
          <div class="d">
            <div class="e">
              <div class="f">
                <div class="g"></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </body>
</html>
```

**test.css:**

```css
* {
  display: block;
  padding: 12px;
}
.a {
  background: #ff0000;
}
.b {
  background: #ffa500;
}
.c {
  background: #ffff00;
}
.d {
  background: #008000;
}
.e {
  background: #0000ff;
}
.f {
  background: #4b0082;
}
.g {
  background: #800080;
}
```

This will produce a PNG image with nested colored rectangles.

## Dependencies

- **`getopts`** - Command-line argument parsing
- **`image`** - PNG image encoding

## TODO / Planned Features

The following features are planned for future implementation:

1. **Cascading** - Proper CSS cascade order and specificity resolution
2. **Initial and/or computed values** - Default values for CSS properties
3. **Inheritance** - CSS property inheritance from parent to child elements
4. **The `style` attribute** - Support for inline styles via the HTML `style` attribute
5. **Collapsing vertical margins** - CSS margin collapsing behavior
6. **Supporting alpha** - Alpha compositing and transparency support (RGBA colors, opacity)

## Limitations

- Only supports block-level layout (no inline layout yet)
- No text rendering
- No support for many CSS properties
- No z-index support
- No transparency/alpha blending
- Simplified CSS selector matching
- No support for CSS inheritance or cascading

## References

This project is based on the tutorial series by Matt Brubeck:

- [Part 1: Getting started](https://limpet.net/mbrubeck/2014/08/08/toy-layout-engine-1.html)
- [Part 2: HTML](https://limpet.net/mbrubeck/2014/08/11/toy-layout-engine-2-html.html)
- [Part 3: CSS](https://limpet.net/mbrubeck/2014/08/13/toy-layout-engine-3-css.html)
- [Part 4: Style](https://limpet.net/mbrubeck/2014/09/17/toy-layout-engine-4-style.html)
- [Part 5: Boxes](https://limpet.net/mbrubeck/2014/10/13/toy-layout-engine-5-boxes.html)
- [Part 6: Block layout](https://limpet.net/mbrubeck/2014/10/14/toy-layout-engine-6-block.html)
- [Part 7: Painting 101](https://limpet.net/mbrubeck/2014/11/05/toy-layout-engine-7-painting.html)

## License

This is an educational project based on the tutorial series mentioned above.
