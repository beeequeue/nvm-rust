# nvm(-rust)

Cross platform nvm that doesn't suck™

## Feature Comparison

| | **nvm-rust** | [nvm-windows](https://github.com/coreybutler/nvm-windows) | [nvm](https://github.com/nvm-sh/nvm) |
| ---: | :---: | :---: | :---: | 
| Platforms | [Rust Platforms](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-1) | Windows | POSIX |
| [Range matching](#range-matching) | ✅ | ❌ | ✅ |
| [.nvmrc](#nvmrc) | 🔧 | ❌ | ✅ |
| [Default global packages](#default-global-packages) | 🔧 | ❌ | ✅ |
| Node <4 | ✅* | ✅ | ✅ |
| Disabling nvm temporarily | ❌ | ✅ | ✅ |
| Caching | ❌ | ❌ | ✅ |
| Aliases | ❌ | ❌ | ✅ |

\*not supported, might work?

### Range Matching

Allowing you to not have to write out the full versions when running a command.

For example:

- `nvm install 12` will install the latest version matching `12`, instead of `12.0.0`.
- `nvm install "12 <12.18"` will install the latest `12.17.x` version, instead of just giving you an error.
- `nvm use 12` will use the newest installed `12.x.x` version instead of `12.0.0` (and giving you an error).

### .nvmrc

### Default global packages
