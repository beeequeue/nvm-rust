# nvm(-rust)

Cross platform nvm that doesn't suck™

## Installation

### Binaries

1. Download binary for your OS from the [Releases](https://github.com/BeeeQueue/nvm-rust/releases)
2. Rename the file to `nvm` and place it somewhere in your `$PATH`
3. Enjoy?

#### Note for Windows

_It does not allow creating the symlinks this program uses without either Admin access or Developer Mode._

_Either run the program as Administrator or [enable Developer Mode](https://docs.microsoft.com/en-us/windows/apps/get-started/enable-your-device-for-development#active-developer-mode)_

_[Read more about it here](https://blogs.windows.com/windowsdeveloper/2016/12/02/symlinks-windows-10)_

### Cargo

```shell
cargo install nvm-rust
```

## Feature Comparison

|                                                                        |  **nvm-rust**   | [nvm-windows](https://github.com/coreybutler/nvm-windows) | [nvm](https://github.com/nvm-sh/nvm) |
|-----------------------------------------------------------------------:|:---------------:|:---------------------------------------------------------:|:------------------------------------:|
|                                                              Platforms | Win, Mac, Linux |                          Windows                          |                POSIX                 |
|                                      [Range matching](#range-matching) |        ✅        |                             ❌                             |                  ✅                   |
| [Version files](#version-files-packagejsonengines-nvmrc-tool-versions) |        ✅        |                             ❌                             |                  ✅                   |
|                    [Default global packages](#default-global-packages) |       🔧        |                             ❌                             |                  ✅                   |
|                                                                Node <4 |       ✅*        |                             ✅                             |                  ✅                   |
|                                              Disabling nvm temporarily |        ❌        |                             ✅                             |                  ✅                   |
|                                                                Caching |        ❌        |                             ❌                             |                  ✅                   |
|                                                                Aliases |        ❌        |                             ❌                             |                  ✅                   |

**not supported, might work?

### Range Matching

Allowing you to not have to write out the full versions when running a command.

For example:

- `nvm install 12` will install the latest version matching `12`, instead of `12.0.0`.
- `nvm install "12 <12.18"` will install the latest `12.17.x` version, instead of just giving you an error.
- `nvm use 12` switch use the newest installed `12.x.x` version instead of `12.0.0` (and most likely giving you an error, who has that version installed?).

### Version files (`package.json#engines`, `.nvmrc`, `.tool-versions`)

If a version is not specified for the `use` and `install` commands nvm-rust will look for and parse any files containing Node version specifications amd use that!

nvm-rust handles files containing ranges, unlike [nvm](https://github.com/nvm-sh/nvm).

e.g.

```
// package.json
{
  ...
  "engines": {
    "node": "^14.17"
  }
  ...
}

# Installs 14.19.3 as of the time of writing
$ nvm install
```

The program will use the following file priority:

1. `package.json#engines`
2. `.nvmrc`
3. `.node-version`
4. [`.tool-versions` from `asdf`](https://asdf-vm.com/guide/getting-started.html#local)

### Default global packages


## Development

This project uses [Task](https://taskfile.dev/installation) to execute various development commands.

e.g. to run a command via a debug build, run:

```shell
task run -- install 12
```

To build a release artifact, run:

```shell
task build:release
```

You can find all the commands in the [Taskfile](./Taskfile.yml).
