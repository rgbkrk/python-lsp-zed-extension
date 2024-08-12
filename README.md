# Python LSP Server Zed Extension

This extension uses the [Python Language Server](https://github.com/python-lsp/python-lsp-server) to provide language support for Python in Zed. It has providers for `jedi`, `mccabe`, `pycodestyle`, `pydocstyle`, `pyflakes`, `pylint`, `rope`, `yapf`, `mypy`, and more.

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
