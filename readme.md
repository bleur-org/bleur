<p align="center">
    <img src=".github/assets/header.png" alt="Bleur's {Buddy}">
</p>

<p align="center">
    <h3 align="center">Template manager & buddy for bleur-org/templates.</h3>
</p>

<p align="center">
    <img align="center" src="https://img.shields.io/github/languages/top/bleur-org/bleur?style=flat&logo=rust&logoColor=000000&labelColor=ffffff&color=ffffff" alt="Top Used Language">
</p>

This is template manager/assistant for creating/managing your own tempalte or bootstrapping a project using ready-to-go templates from remote. Also, this project itself was bootstrapped using Bleur's templates!

## Development

The project has `shell.nix` which has development environment preconfigured already for you. Just open your
terminal and at the root of this project:

```bash
# Open in bash by default
nix develop

# If you want other shell
nix develop -c $SHELL

# After entering Nix development environment,
# inside the env, you can open your editor, so
# your editor will read all $PATH and environmental
# variables, also your terminal inside your editor
# will adopt all variables, so, you can close terminal.

# Neovim
vim .

# VSCode
code .

# Zed Editor
zed .
```

The development environment has whatever you may need already, but feel free to add or remove whatever
inside `shell.nix`.

## Building

Well, there are two ways of building your project. You can either go with classic `cargo build` way, but before that, make sure to enter development environment to have cargo and all rust toolchain available in your PATH, you may do like that:

```bash
# Entering development environment
nix develop -c $SHELL

# Compile the project
cargo build --release
```

Or, you can build your project via nix which will do all the dirty work for you. Just, in your terminal:

```bash
# Build in nix environment
nix build

# Executable binary is available at:
./result/bin/bleur
```

## FAQ

### Why not use default.nix for devShell?

There's been cases when I wanted to reproduce totally different behaviors in development environment and
production build. This occurs quite a lot lately for some reason and because of that, I tend to keep
both shell.nix and default.nix to don't mix things up.

## Thanks

- [Orzklv](https://github.com/orzklv) - For starting/maintaining.
- [Kei](https://github.com/thelissimus) - For helping with structure.
- [Let-rec](https://github.com/let-rec) - For improvements/contributions.
- [Orzklv's Vocaloid Playlist](https://music.apple.com/gb/playlist/vocaloid-songs/pl.u-GgA5YE5io7P71kE) - For keeping my (orzklv's) sanity.

## License

This project is dual licensed under the MIT License and Apache-2.0 - see the [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) files for details. Proudly written and developed by human, no usage of AI (LLM) at all, only brain and compiler.

<p align="center">
    <a href="https://notbyai.fyi/app/draft/1445">
      <img src=".github/assets/footer.png" alt="Bleur's {Buddy}">
    </a>
</p>
