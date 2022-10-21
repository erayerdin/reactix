# Reactix

![version](https://img.shields.io/github/v/release/erayerdin/reactix?label=version&style=flat-square)
![license](https://img.shields.io/badge/license-WTFPL-green?style=flat-square)

A Rust Actix template using React and Actix.

## Features

 - React with Typescript
 - Error management with [anyhow][anyhow_crate] and [thiserror][thiserror_crate]
 - Embedded migrations, pooling and database management with [sqlx][sqlx_crate]
 - Configuration management with [envy][envy_crate] and [dotenvy][dotenvy_crate].
 - Logging implementation with [log][log_crate] and [fern][fern_crate].
 - Actix features
    - Rate limiting and throttling with [governor][governor_crate].
    - Cookie, session and identity management with [actix-session][actix-session_crate] and [actix-identity][actix-identity_crate]
    - Form management with [actix-multipart-extract][actix-multipart-extract_crate].
 - VSCode Stuff
   - [Recommended Extensions](.vscode/extensions.json) will notify you to install when first launched.
   - [Settings](.vscode/settings.json) will hide cluttering files and directories from file dock, add license header to every file when first created etc.
   - [Tasks](.vscode/tasks.json), with which you can watch file change and recompile the project on save

[anyhow_crate]: https://crates.io/crates/anyhow
[thiserror_crate]: https://crates.io/crates/thiserror
[sqlx_crate]: https://crates.io/crates/sqlx
[envy_crate]: https://crates.io/crates/envy
[dotenvy_crate]: https://crates.io/crates/dotenvy
[log_crate]: https://crates.io/crates/log
[fern_crate]: https://crates.io/crates/fern
[governor_crate]: https://crates.io/crates/governor
[actix-session_crate]: https://crates.io/crates/actix-session
[actix-identity_crate]: https://crates.io/crates/actix-identity
[actix-multipart-extract_crate]: https://crates.io/crates/actix-multipart-extract

## Requirements

 - NPM/NodeJS
 - [SQLx CLI](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md#with-rust-toolchain)
 - [Cargo Watch](https://github.com/watchexec/cargo-watch#install)

## Initial Configuration

`xdg-open` command below launches the given file/directory with its default application. For example, the default app for a plain text or source code file might be Gedit in Gnome or Kwrite in KDE Plasma environment. Or you can explicitly define the editor you'd like to use.

``` bash
# clone this repo
git clone https://github.com/erayerdin/reactix

# configure or review vscode settings and tasks for your needs
xdg-open .vscode/extensions.json
xdg-open .vscode/tasks.json

# edit license and readme on your needs
xdg-open LICENSE
xdg-open README.md

# edit cargo metadata
xdg-open Cargo.toml

# configure env files
mv .example.env .env
mv .example.build.env .build.env
xdg-open .env
xdg-open .build.env
```

From here on, there are some things you might prefer to do with VSCode.

If you have installed [Todo Tree](https://marketplace.visualstudio.com/items?itemName=Gruntfuggly.todo-tree) extension, which is prompted to be installed at the first launch due to [recommended extensions file](.vscode/extensions.json), you probably need to check its panel in VSCode (the icon on the left with a tree and a tick mark in it). It will guide you to what needs to be changed in order to get this template working.

# License

The source code here is licensed under [Do What the Fuck You Want to Public License](http://www.wtfpl.net/).