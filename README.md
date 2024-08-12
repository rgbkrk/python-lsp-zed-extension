# Python LSP Server Zed Extension

This extension uses the [Python Language Server](https://github.com/python-lsp/python-lsp-server) to provide language support for Python in Zed. It has providers for `jedi`, `mccabe`, `pycodestyle`, `pydocstyle`, `pyflakes`, `pylint`, `rope`, `yapf`, `mypy`, and more.

## Installation

Before installing this extension, install the [Python Language Server](https://github.com/python-lsp/python-lsp-server) and ensure that it is on your `PATH`. Example, using `pipx`:

```
pipx install "python-lsp-server[all]"
```

All extensions to the `python-lsp-server` must be installed to the same virtualenv that you install the server into.


## Configuration

Use any of the [configuration settings](https://github.com/python-lsp/python-lsp-server/blob/develop/CONFIGURATION.md) that pylsp supports within your zed config under `pylsp.settings`. Here's an example that disables the `E501` line length warning:

```jsonc
"pylsp": {
  "settings": {
    "plugins": {
      "pycodestyle": {
        "enabled": true,
        "ignore": ["E501"]
      }
    }
  }
}
```
