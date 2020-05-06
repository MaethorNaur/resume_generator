# Resume Generator

Generate a resume directly into PDF using `JSONResume` format.

The input can either be in `JSON` or in `Toml`.

## Install

```sh
cargo install --git  https://github.com/MaethorNaur/resume_generator
```

## Usage

```sh
resume-generator -h
resume-generator 0.1.0
Resume generator

USAGE:
    resume-generator [FLAGS] [OPTIONS] <filename> [output]

FLAGS:
    -h, --help       Prints help information
    -q, --quiet      Pass many times for less log output
    -V, --version    Prints version information
    -v, --verbose    Pass many times for more log output

OPTIONS:
        --gs <path>    Ghostscript executable

ARGS:
    <filename>    Resume to generate, (Json or Toml)
    <output>      Generated resume [default: resume.pdf]

```
