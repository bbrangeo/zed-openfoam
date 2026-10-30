# OpenFOAM extension for Zed

[OpenFOAM](https://www.openfoam.com/) dictionary support for the [Zed editor](https://zed.dev): syntax highlighting, code folding, outline and a language server (completion, hover docs, diagnostics) for OpenFOAM case files — `controlDict`, `fvSchemes`, `fvSolution`, `constant/*`, boundary condition files, etc.

## Features

- **Syntax highlighting** via the [`tree-sitter-foam`](https://github.com/FoamScience/tree-sitter-foam) grammar
- **Code folding** and bracket matching for dictionaries and lists
- **Code outline** (dictionaries and key–value pairs)
- **Auto-indentation**
- **Language server** ([`foam-language-server`](https://github.com/FoamScience/foam-language-server), installed automatically via npm) providing:
  - completion (macros, keywords)
  - hover documentation
  - diagnostics (tree-sitter syntax errors + OpenFOAM solver log validation)
  - document symbols, highlights, references, rename, folding ranges, links

## Requirements

- [Node.js](https://nodejs.org/) ≥ 18 on your `PATH` (used to run `foam-language-server`; Zed downloads and installs the npm package automatically on first launch)

## Installation

### From the Zed extensions directory

Open the command palette (`cmd-shift-p`) → `zed: extensions` → search for **OpenFOAM**.

### From source (dev extension)

```sh
git clone https://github.com/bbrangeo/zed-openfoam.git
cd zed-openfoam
cargo build --target wasm32-wasip2
```

Then in Zed: command palette → `zed: install dev extension` and select this directory.

## Usage

Open an OpenFOAM **case directory** as a Zed project (`File → Open Folder…`):

```
myCase/
├── 0/            (or 0.orig, 0/)
│   ├── U
│   ├── p
│   └── ...
├── constant/
│   └── turbulenceProperties
└── system/
    ├── controlDict
    ├── fvSchemes
    └── fvSolution
```

Opening the case folder (not just a single file) lets the language server index the whole case, which is required for cross-file completions and solver diagnostics.

> **Note:** published `foam-language-server` releases on npm (≤ 0.4.6) lag behind upstream `master` and contain bugs that crash or fail Zed's `InitializeResult` deserialization. This extension patches the compiled server in place at startup with the fixes from upstream `master`, so everything works out of the box.

## Configuration

Per-project LSP settings can be tuned in `.zed/settings.json`:

```json
{
  "lsp": {
    "foam-ls": {
      "settings": {
        "validator": {
          "fatalError": "error",
          "warning": "warning"
        }
      }
    }
  }
}
```

## Development

- Grammar: [`FoamScience/tree-sitter-foam`](https://github.com/FoamScience/tree-sitter-foam) pinned at rev `f72e493dadcca6a334c113b4cbaf864460541de4` (fixes a SIGSEGV in the scanner)
- Language server: [`FoamScience/foam-language-server`](https://github.com/FoamScience/foam-language-server)
- Queries live in [`languages/OpenFOAM/`](languages/OpenFOAM/) (`highlights.scm`, `folds.scm`, `injections.scm`, `locals.scm`, `outline.scm`, `indents.scm`)
- Extension entry point: [`src/lib.rs`](src/lib.rs)

## Icon

The extension icon is derived from the [OpenFOAM wordmark](https://doc.openfoam.com/2606/img/openfoam-wordmark.svg) (OpenFOAM is a registered trademark of OpenCFD Ltd / ESI Group; this extension is not affiliated with or endorsed by OpenCFD).

## License

[MIT](LICENSE)
