# Contribution Guide

You should use macOS to be able to build for both android and iOS.

## Update Submodules

```sh
git submodule update --recursive --remote
```

## Android

```bash
brew install openssl@3
export OPENSSL_DIR=$(brew --prefix openssl)
```
