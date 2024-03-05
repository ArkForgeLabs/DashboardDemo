# Dashboard demo

This project is a demo of a dashboard being built using [Blue Engine](https://github.com/BlueEngine) as base and [Egui](https://github.com/emilk/egui) as the UI library.

The goal is to use the dashboard in mainline commercial products of `ArkForge` and to show a template base for anyone else desiring to build custom graphics using the same way. Updates will be slow on this project, and the content is subject to change. Respective licenses for assets used are available in their folders.

## Installation

You can start by forking, cloning, or using the template button on this repository to create a new project. Once you have it on disk, open a terminal and run `cargo run` to start the project.

The default `debug_crates` folder contains `BlueEngine` and `BlueEngineUtilities` which contains the `Egui` library. The purpose of the folder is to help in dynamically linking them and decrease the build times massively. **You should swap them to official crate version on release.**

## Contributing

If you want to contribute, please follow the [contributing guidelines](https://github.com/AryanpurTech/BlueEngine/blob/master/CONTRIBUTING.md).
