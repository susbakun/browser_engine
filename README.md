# Browser Engine

A toy browser rendering engine written in Rust that parses HTML and CSS, applies styling, performs layout calculations, and displays the result in a live window.

This project is based on the excellent tutorial series ["Let's build a browser engine!"](https://limpet.net/mbrubeck/2014/11/05/toy-layout-engine-7-painting.html) by Matt Brubeck.

## Overview

This browser engine implements a simplified rendering pipeline that transforms HTML and CSS into a visual representation. The engine follows the standard browser rendering pipeline:

1. **HTML Parsing** — Parses HTML into a DOM tree
2. **CSS Parsing** — Parses CSS rules and declarations
3. **Style** — Matches CSS rules to DOM nodes and builds a style tree
4. **Layout** — Calculates the position and size of each element (block layout)
5. **Painting** — Rasterizes the layout tree into a pixel buffer
6. **Window** — Displays the pixel buffer in a live window (winit + pixels)

## Features

### Currently Implemented

### Currently Implemented

- ✅ HTML parsing into a DOM tree
- ✅ CSS parsing (selectors, declarations, values)
- ✅ Style tree construction with rule matching
- ✅ Block-level layout algorithm
- ✅ Basic painting/rasterization
- ✅ Background color rendering
- ✅ Border rendering
- ✅ Padding support
- ✅ Nested block elements
- ✅ Image rendering (`<img>`), including local files and remote HTTP/HTTPS images
- ✅ Text rendering
- ✅ Alpha channel support and transparency blending
- ✅ Hex color parsing with alpha channel (`#rrggbbaa` format)
- ✅ RGB function support with optional alpha channel (`rgb(r, g, b)` and `rgb(r, g, b / alpha)`)
- ✅ Inline CSS styles via the HTML `style` attribute
- ✅ `display: none` support (e.g. hide `<head>`)
- ✅ Live window display (800×600)

## Architecture

The project is organized into several modules:

- **`dom.rs`** — DOM node structure and tree representation
- **`html.rs`** — HTML parser
- **`css.rs`** — CSS parser and value types
- **`style.rs`** — Style tree construction and CSS rule matching
- **`layout.rs`** — Block layout algorithm and box model calculations
- **`painting.rs`** — Display list generation and rasterization
- **`window.rs`** — Live window creation and pixel buffer display
- **`cli.rs`** — Command-line argument parsing
- **`image.rs`** — Image loading, decoding, resizing, and rendering

## Usage

### Prerequisites

- Rust (latest stable version recommended)
- Cargo

### Building

```bash
cargo build
```

### Running

The program accepts command-line arguments for HTML and CSS files and opens a window with the rendered result:

```bash
cargo run -- --html <html_file> --css <css_file>
```

**Default behavior** (if no arguments provided):

- HTML: `./test.html`
- CSS: `./test.css`

**Example:**

```bash
cargo run -- -h test.html -c test.css
```

Press **Escape** or close the window to exit.

### Example Input

**test.html:**

```html
<head>
  <title>Document</title>
</head>
<body>
  <div style="background: #ff000095;">
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
```

**test.css:**

```css
head {
  display: none;
}
* {
  display: block;
  padding: 12px;
}

div.b {
  background: #ffa500a5;
}
div.c {
  background: #ffff00;
}
div.d {
  background: #008000;
}
div.e {
  background: #0000ff;
}
div.f {
  background: #4b0082;
}
div.g {
  background: rgb(10 20 30 / 0.25);
}
```

This renders nested colored rectangles with semi-transparent backgrounds in the window.

**Color Formats:**

You can use hex colors with an alpha channel by using 8 hex digits instead of 6:

```css
.semi-transparent {
  background: #ff000080; /* Red with 50% opacity (128/255) */
}
```

You can also use the `rgb()` function with optional alpha channel:

```css
.red {
  background: rgb(255, 0, 0); /* Opaque red */
}

.semi-transparent-blue {
  background: rgb(0, 0, 255 / 0.5); /* Blue with 50% opacity */
}
```

The alpha value in `rgb()` should be a float between 0.0 (fully transparent) and 1.0 (fully opaque). The engine performs alpha compositing, blending semi-transparent colors with the background colors underneath them.

**Inline Styles:**

You can use inline CSS styles directly in HTML elements using the `style` attribute:

```html
<div style="background: #00ff00; padding: 20px;">
  This div has an inline style
</div>
```

Inline styles have higher specificity than stylesheet rules and will override matching CSS rules.

## Dependencies

- **`anyhow`** — Error handling
- **`getopts`** — Command-line argument parsing
- **`pixels`** — Pixel buffer rendering
- **`winit`** — Cross-platform window creation
- **`winit_input_helper`** — Keyboard and window input handling
- **`image`** — Image decoding and resizing
- **`reqwest`** — Downloading remote images over HTTP/HTTPS
- **`fontdue`** — Text rasterization

## TODO / Planned Features

The following features are planned for future implementation:

1. **Cascading** — Proper CSS cascade order and specificity resolution
2. **Initial and/or computed values** — Default values for CSS properties
3. **Collapsing vertical margins** — CSS margin collapsing behavior
4. **Inline layout** — Layout for inline elements

## Limitations

- Only supports block-level layout (inline layout is not yet implemented)
- Text rendering currently supports **Roboto** only
- Limited CSS property support
- Simplified CSS selector matching
- No z-index or stacking contexts
- Fixed viewport size (800×600)

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
