Design doc - https://docs.google.com/document/d/1Fmn5A9aLNOFJ0A6bqsuo7XWaz2jTZ0GFZmN9Elpx3E4/edit?usp=sharing
Miro design board - https://miro.com/welcomeonboard/aE5wcUFEUGJybGdJT1NHcXBybzBKanpWMDhXaEtNUWJlSWNDcmhvSVEzeFFEam5la3ZmZFlPWGc4OVFSS1k2UHwzNDU4NzY0NTk0ODE4NTA1NTkxfDI=?share_link_id=472846460666
Global team info (time zones etc) - https://docs.google.com/spreadsheets/d/1XoyHA536f8EukFLZK38vWAHRh7u84ypvU8089tvwB3E/edit?usp=sharing



_Brought to you by the Bevy Jam working group._

# Bevy Quickstart

This template is a great way to get started on a new [Bevy](https://bevyengine.org/) game—especially for a game jam!
Start with a [basic project structure](#write-your-game) and [CI / CD](#release-your-game) that can deploy to [itch.io](https://itch.io).
You can [try this template in your web browser!](https://the-bevy-flock.itch.io/bevy-quickstart)

Don't want to read through the whole README? [@ChristopherBiscardi](https://github.com/ChristopherBiscardi) made a video on how to use the template from start to finish:

[<img src="./docs/thumbnail.png" width=40% height=40% alt="A video tutorial for bevy_quickstart"/>](https://www.youtube.com/watch?v=ESBRyXClaYc)

## Prerequisites

We assume that you know how to use Bevy already and have seen the [official Quick Start Guide](https://bevyengine.org/learn/quick-start/introduction/).

## Create a new game

Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate) and run the following commands:

```sh
cargo generate TheBevyFlock/bevy_quickstart --branch cargo-generate
git branch --move main
```

Then [create a GitHub repository](https://github.com/new) and push your local repository to it.

<details>
  <summary>This template can also be set up manually.</summary>

Navigate to the top of [this GitHub repository](https://github.com/TheBevyFlock/bevy_quickstart/) and select `Use this template > Create a new repository`:

![UI demonstration](./docs/readme-manual-setup.png)

Clone your new Github repository to a local repository and push a commit with the following changes:

- Delete `LICENSE`, `README`, and `docs/` files.
- Search for and replace instances of `bevy_quickstart` with the name of your project.
- Adjust the `env` variables in [`.github/workflows/release.yaml`](./.github/workflows/release.yaml).

</details>

## Write your game

The best way to get started is to play around with what you find in [`src/game/`](./src/game).

This template comes with a basic project structure that you may find useful:

| Path                                     | Description                                           |
|------------------------------------------|-------------------------------------------------------|
| [`src/lib.rs`](./src/lib.rs)             | App setup                                             |
| [`src/screen/`](./src/screen)            | Splash screen, title screen, playing screen, etc.     |
| [`src/game/`](./src/game)                | Game mechanics & content (replace with your own code) |
| [`src/ui/`](./src/ui)                    | Reusable UI widgets & theming                         |
| [`src/dev_tools.rs`](./src/dev_tools.rs) | Dev tools for dev builds                              |

Feel free to move things around however you want, though.

If you are new to Bevy, the patterns used in this template may look a bit weird at first glance.
See our [Design Document](./docs/design.md) for more information on how we structured the code and why.

> [!Tip]
> Be sure to check out the [3rd-party tools](./docs/tooling.md) we recommend!

## Run your game

Running your game locally is very simple:

- Use `cargo run` to run a native dev build.
- Use [`trunk serve`](https://trunkrs.dev/) to run a web dev build.

If you're using [VS Code](https://code.visualstudio.com/), this template comes with a [`.vscode/tasks.json`](./.vscode/tasks.json) file.

<details>
  <summary>Run release builds</summary>

- Use `cargo run --profile release-native --no-default-features` to run a native release build.
- Use `trunk serve --release --no-default-features` to run a web release build.

</details>

<details>
    <summary>(Optional) Improve your compile times</summary>

[`.cargo/config_fast_builds.toml`](./.cargo/config_fast_builds.toml) contains documentation on how to set up your environment to improve compile times.
After you've fiddled with it, rename it to `.cargo/config.toml` to enable it.

</details>

## Release your game

This template uses [GitHub workflows](https://docs.github.com/en/actions/using-workflows) to run tests and build releases.
See [Workflows](./docs/workflows.md) for more information.

## Known Issues

There are some known issues in Bevy that require some arcane workarounds.
To keep this template simple, we have opted not to include those workarounds.
You can read about them in the [Known Issues](./docs/known-issues.md) document.

## License

The source code in this repository is licensed under any of the following at your option:

- [CC0-1.0 License](./LICENSE-CC0-1.0.txt)
- [MIT License](./LICENSE-MIT.txt)
- [Apache License, Version 2.0](./LICENSE-Apache-2.0.txt)

We hold no patent rights to anything presented in this repository.

## Credits

The [assets](./assets) in this repository are all 3rd-party. See the [credits screen](./src/screen/credits.rs) for more information.
