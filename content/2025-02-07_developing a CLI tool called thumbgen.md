+++
title = "I am developing a CLI tool called thumbgen"
description = "I would like to introduce a CLI tool I am developing called thumbgen."
slug = "I am developing a CLI tool called thumbgen that generates thumbnail images"
draft = false
[taxonomies]
tags = ["Rust", "thumbgen", "CLI"]
languages = ["English"]
+++

## About this tool

CLI tool to generate images for thumbnails.

{{ card(title="thumbgen | GitHub", url="https://github.com/Daiki48/thumbgen") }}

## Why

Because I want to generate thumbnail images for my personal website.
I would like to generate images to be set in the OGP on my personal website.
Right now, version `0.6.0` is the latest version.

{{ card(title="thumbgen | crates.io", url="https://crates.io/crates/thumbgen") }}

## Currently available features

The first step is to install thumbgen.
Now it can be installed only with `cargo` .

```sh
cargo install thumbgen
```

In order to use the tool, initial setup is required.

```sh
thumbgen init
```

Generate a `.thumbgen` directory in the path where you run it.
A `config.toml` is generated in it.
Version `0.6.0` contains the following default settings.

```toml
# This config is setup for thumbgen

[meta]
title = 'Hello. I love Rust language. Rust is best for programming. I writing Rust at thumbgen. My Github username is Daiki48.'
username = 'Daiki48'

[design]
background_color = [34, 39, 46, 255]
box_color = [55, 63, 74, 255]
font_color = [202, 204, 202, 255]
```

This setting will be changed in the future.
You can set the meta information and design of the generated thumbnails.

To generate an image, run the create command.

```sh
thumbgen create
```

Images are generated in the `.thumbgen` directory.

Right now, users do not have the flexibility to customize.

## Future tasks

- [ ] Set user icon
- [ ] `config.toml` overwrite
- [ ] Customize thumbnail size
- [ ] Customize font style
- [ ] Documentation of user guides

## Please try it!

I started developing this tool to write `Rust` .
I am learning `Rust`.
If you have a feature you would like to see added, or if a bug occurs, please create an Issue.

Thank you :+1:
