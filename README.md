# dprint-plugin-rusty

dprint-plugin-rusty provides three formatter plugins for dprint: one for PHP files using mago, one for Lua files using StyLua, and one for Markdown using rumdl. All are compiled to WebAssembly and plug into dprint like any other plugin.

##### Why?

Using dprint plugins lets you keep a single, consistent formatting workflow across polyglot repos. The mago plugin applies mago’s PHP formatter, the StyLua plugin formats Lua with the same rules as StyLua, and the rumdl plugin applies rumdl’s fixer/linter to Markdown content.

## Usage

### mago (PHP)

Add the mago plugin to your **dprint** configuration to format PHP files.

```json
{
  "$schema": "https://dprint.dev/schemas/v0.json",
  "plugins": [
    "https://github.com/mridang/dprint-plugin-rusty/releases/download/v1.0.0/mago.wasm"
  ],
  "includes": [
    "**/*.php",
    "**/*.phtml"
  ]
}
```

```bash
dprint fmt --log-level=debug
```

#### Options

This plugin exposes line width, indent width, and tabs vs spaces (mirrors dprint’s global settings).

### StyLua (Lua)

Add the StyLua plugin to your **dprint** configuration to format Lua files.

```json
{
  "$schema": "https://dprint.dev/schemas/v0.json",
  "plugins": [
    "https://github.com/mridang/dprint-plugin-rusty/releases/download/v1.0.0/stylua.wasm"
  ],
  "includes": [
    "**/*.lua"
  ]
}
```

```bash
dprint fmt --log-level=debug
```

#### Options

This plugin exposes line width, indent width, tabs vs spaces, and quote style. Other StyLua options use upstream defaults.

### rumdl (Markdown)

Add the rumdl plugin to your **dprint** configuration to format Markdown files.

```json
{
  "$schema": "https://dprint.dev/schemas/v0.json",
  "plugins": [
    "https://github.com/mridang/dprint-plugin-rusty/releases/download/v1.0.0/rumdl.wasm"
  ],
  "includes": [
    "**/*.md",
    "**/*.mdx",
    "**/*.qmd"
  ]
}
```

```bash
dprint fmt --log-level=debug
```

#### Options

This plugin exposes line width and flavor (Markdown dialect). It uses rumdl’s auto-fix pipeline and returns the fixed content.

## Caveats

- Only mago, StyLua, and rumdl are provided (no Ruby formatter).

## Contributing

Contributions are welcome! If you find a bug or have suggestions for improvement, please open an issue or submit a pull request.

## License

Apache License 2.0 © 2025 Mridang Agarwalla
