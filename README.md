## dm-Reverse

A reverse delay effect written in Rust.
The effect can be compiled to a [lv2](./lv2) or [vst](./vst) plugin.
This plugin has been written primarily to run on [Mod devices](https://moddevices.com/). And because I mainly use this for guitar it's just mono for now.

## Table of contents:

- [Mod devices installation](#Mod-devices-installation)
- [LV2 installation](#LV2-installation)
- [VST installation](#VST-installation)
- [License](#License)

## Mod devices installation

You can find the plugin for the Mod Dwarf [here](./lv2/dm-Reverse.lv2/).

For Mod Duo, follow the [lv2 instructions](#LV2-installation) first. Then finish the instructions below.

- Copy the .lv2 folder into your Mod:

  ```
  scp -rp <path to dm-Reverse.lv2> root@192.168.51.1:/root/.lv2
  ```

- Enter Mod password
- Reboot Mod

## LV2 installation

In order to build the binaries you need to have Docker installed. If so, proceed with the following steps:

- Run `./build-lv2.sh` in the root directory.
- Copy/paste the binary of the target platform from the `./lv2/out` directory into `./lv2/dm-Reverse.lv2`

## VST installation

First go to the [vst folder](./vst).

Windows:

1. Run `cargo build --release`
2. Copy libdm_reverse.dll in /target/release to your vst plugin folder

Mac

1. Run `cargo build --release`
2. Run `./osx_vst_bundler.sh dm-Reverse target/release/libdm_reverse.dylib`
3. Copy dm-Reverse.vst in the root of this folder to your vst plugin folder

## License

[![CC BY 4.0][cc-by-shield]][cc-by]

This work is licensed under a
[Creative Commons Attribution 4.0 International License][cc-by].

[![CC BY 4.0][cc-by-image]][cc-by]

[cc-by]: http://creativecommons.org/licenses/by/4.0/
[cc-by-image]: https://i.creativecommons.org/l/by/4.0/88x31.png
[cc-by-shield]: https://img.shields.io/badge/License-CC%20BY%204.0-lightgrey.svg
