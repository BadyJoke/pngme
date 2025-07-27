# 📦 PNGme

> Command line program that lets you hide secret messages in PNG files.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rust-stable-brightgreen)

This CLI has been made following the [PNGme: An Intermediate Rust Project](https://jrdngr.github.io/pngme_book/introduction.html) book.

---

## ✨ Features

- Encode and decode secret messages
- Remove secret messages
- Print chunks of the PNG file

---

## 🚀 Installation

### From source

Clone `pngme` from git:

```sh
git clone https://github.com/BadyJoke/pngme.git
```

Install it using cargo:

```sh
cd pngme && cargo install project-name
```

## ⚡️ Usage

### Encode a secret message into a file

```sh
pngme encode <FILE_PATH> <CHUNK_TYPE> <MESSAGE>
```

Example:

```sh
pngme encode file.png mySc "Secret message hiding in a PNG file"
```

### Decode a secret message into a file

```sh
pngme decode <FILE_PATH> <CHUNK_TYPE>
```

Example:

```sh
pngme encode file.png mySc 
```

### Remove a secret for a file

```sh
pngme remove <FILE_PATH> <CHUNK_TYPE>
```

Example:

```sh
pngme encode file.png mySc 
```

### Print chunks from a file

```sh
pngme print <FILE_PATH>
```

Example:

```sh
pngme encode file.png
```

## 📄 License

[MIT](./LICENSE)