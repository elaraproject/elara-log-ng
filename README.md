# elara-log

> :warning: **IMPORTANT:** Project Elara has switched over to the open-source forge [Codeberg](https://codeberg.org/). The Project Elara repositories have been moved to [this Codeberg page](https://codeberg.org/elaraproject/). **This GitHub repository is no longer maintained**.

---

> **Note:** This library is listed under the name `elara-log-ng` on [crates.io](https://crates.io/crates/elara-log-ng), to avoid a naming conflict with [a previous version](https://crates.io/crates/elara_log) of this library. We understand this can cause a lot of confusion, so please refer to the detailed instructions in our [docs](https://docs.rs/elara-log-ng) on how to use the library and install it properly.

`elara-log` is a minimalist and lightweight logger, developed for [Project Elara](https://github.com/elaraproject/)'s suite of libraries and applications. See the [docs](https://docs.rs/elara-log-ng) for a usage overview.

> `elara-log` is **public domain software** like the rest of [Project Elara](https://github.com/elaraproject/), meaning it is essentially **unlicensed software**, so you can use it for basically any project you want, _however_ you want, with or without attribution.

## Platform compatibility

The library should work fine on most relatively modern PCs (2010+), and probably also works for a lot of older systems as well, since it just needs support for [ANSI escape codes](https://en.wikipedia.org/wiki/ANSI_escape_code) (which is generally supported on most systems). For specific platforms:

- Tested on macOS and Linux and should work without issues on either
- Tested on Windows 10 with Powershell and Git Bash, both work, although the colors do not contrast well with Powershell's default console UI, so it is highly recommended to use the open-source and free [Windows Terminal](https://github.com/microsoft/terminal) and selecting the One Half Dark theme (or in general, any greyish-dark theme) in its appearance settings (which looks nicer anyways)
