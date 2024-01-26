# FGJ-2024

## Setup
> Initial setup will take like 20 minutes thanks to ungodly amount of dependencies for both `bevy` and `wasm-server-runner`

### Native
1. Run `cargo build` and wait for all of the 362 dependencies to compile
2. `cargo run`

### Web-assembly
1. Download trunk `cargo install --locked trunk` and wait for 490 depencies to compile
2. Also download `cargo install --locked wasm-bindgen-cli`
3. Compile to wasm: `cargo build --target wasm32-unknown-unknown --no-default-features`
4. Setup localhost wasm-server: `trunk serve --no-default-features`

### Deploying
1. Create a new tag in the GitHub repo
2. Create a new release with the new tag
3. Set to pre-release
4. Wasm build of the game can be found on github pages


## CI/CD pipeline
> Latest build available on https://tuukezu.github.io/FGJ-2024/
### Documentation for actions:
- https://github.com/actions-rs/toolchain
- https://github.com/jetli/wasm-bindgen-action
- https://github.com/jetli/trunk-action
- https://github.com/peaceiris/actions-gh-pages (this one for some reason really just loves to just break)

### Reasons for why the workflow has imploded...
- cd pipeline didn't feel like triggering due to invalid configuration
- tried to fix said issue and in the process got GitHub actions into endless loop
- fixed said issue but GitHub decided that it still wants to use the old config for god knows what reason
- has to change configuration file name to get it to work again
- runs, but skips the publishing job despite correct configuration according to documentation
- GitHub apparently defaults to 'main' branch even if the config would be set to use 'master' branch
- now we use 'main' instead of 'master'
- still skips the publishing job for some reason
- just remove the branch safety check, who needs that anyways
- fails due to missing 'index.html' file
- also move to using 'trunk' because GitHub actions uses it
- job failes due to permission issues
- documentation recommended fix ends up not fixing the problem
- "just give full permissions to all actions"
- turns out I had forgotten to specify enviroment secret
- random mime-type error on the GitHub pages
- tried to resolve that, now it failed to generate any artifacts
- turns out GitHub pages disliked the trunk generated 'index.html' paths
- found documentation for Trunk.toml and resolved that issue
