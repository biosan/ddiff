# `ddiff`

![Format, Lint & Test](https://github.com/biosan/ddiff/workflows/Format,%20Lint%20&%20Test/badge.svg) ![Build & Release](https://github.com/biosan/ddiff/workflows/Build%20&%20Release/badge.svg) 

A small utility that checks if two folders are the same.


## How it works

`ddiff` is a simple CLI binary written in Rust that computes the hash of **every** file inside two different folders and look for files inside the first folder but not inside the second folder, and viceversa, or files with matching path but different content (hash).


## Why?

Very often I have to check if two folders are the same or not (i.e. copying very important files over the network) and since I’m really paranoid about data integrity, I usually compute the hash of every file in the two folders and compare them.

This works but it’s not easy, it’s not fast and certainly it’s not elegant.

> I know there is a solution to this problem involving `rsync` but IMHO it’s as bad as the “naive”/manual one.

So I decided to build a tool to solve this problem.


## Installation

1. Install from Cargo (requires Rust toolchain):

    ```
    cargo install ddiff
    ```

2. Download binary from [releases](https://github.com/biosan/ddiff/releases) page and add it to you `$PATH`. *Available for linux and macOS on "x86"*


## Usage

```
ddiff 0.1.0

USAGE:
    ddiff <path-a> <path-b>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <path-a>    
    <path-b>
```

*Example output*

```
>>> Files with same path but with different hash

    path         hash A                                                            hash B
    file_random  9b53eed45673c3e91f365f66c9138117bc595888bcdc63f907fafdb28cb72bca  49d138ac765678706c3cc84721af69d66dcb91dfaa71b46e469d296e0788b133


>>> Files in testA/ but not in testB

    path      hash testA/  hash testB
    unique_a  16b3a442c222b958453e73cb818a51a060bed10b9bf6649f2bbb43a9e57bff78


>>> Files in testB but not in testA/

    path      hash testB  hash testA/
    unique_b  16b3a442c222b958453e73cb818a51a060bed10b9bf6649f2bbb43a9e57bff78

ddiff checked 6 files, about 28 B, in 17.220658ms
```


## Hashing

This tool uses the [BLAKE3](https://github.com/BLAKE3-team/BLAKE3) hashing algorithm.

I know... a non-cryptographic hash would be better, but I really like the properties of cryptography hashes and there is a really cool utility called [`b3sum`](https://github.com/blake3-team/blake3/tree/HEAD/b3sum) to verify the hashes.
It’s also fast enough to not be a bottleneck on almost any machine, especially since `ddiff` it’s multithreaded (thanks [Rayon](https://github.com/rayon-rs/rayon))


## Roadmap

- [ ] Add unit tests and more integrations tests
- [ ] Add benchmarks
- [ ] Build ARM binaries **IMPORTANT**
- [ ] Improve documentation and publish it
- [ ] Add a simple installation script
- [ ] Publish on Homebrew and other package managers


## Contributing

Probably no one will ever read this, but in the rare case that you end up here and you want to add some features, improve my code, suggest a new functionality, or more probably to fill up a issue to fix a bug, etc., in any case you are welcome to make PRs, fill issues, or send me a mail.

I'm also very interested in real-world test cases and usage scenarios. Let me know if this small utility was useful to you or if you have any idea on how to improve it.


## Contributing

### Git hooks

This repository has git pre-commit hook to enforce good formatting, code linting, and testing on developer side (thanks [`cargo-husky`](https://github.com/rhysd/cargo-husky)), the same rules will be applied on GitHub Actions.


### Conventional commits

This repo follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) even if is not enforced by a commit message hook.


### Continuous Integration/Delivery

This project use GitHub Actions for CI and building releases.

Thanks to the awesome [`action-rs`](https://github.com/action-rs) project.


## License

This project is licensed under the [MIT license](https://choosealicense.com/licenses/mit/).

