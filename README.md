# Photos.network

[![License](https://img.shields.io/github/license/photos-network/frontend)](./LICENSE.md)
[![GitHub contributors](https://img.shields.io/github/contributors/photos-network/frontend?color=success)](https://github.com/photos.network/core/graphs/contributors)


Photos.network](https://photos.network) is a free and open source, privacy first, self-hosted photo storage and sharing service for fediverse.

Its core features are:
- Share photos with friends, family or public
- Filter / Search photos by attributes like location or date
- Group photos by their content like people or objects
- Upload photos and videos without resolution or quality constraints


## Frontend

This repository contains the official App-like web frontend. of the project.

It is responsible for interacting with the core system via REST calls.
- **Overview** of the users media items in a grid
- **Albums** the users has access to
- **Upload** new media items
- **Details** of items like location, date and time taken.


## 🧩 Contribution

This is a free and open project and lives from contributions of the community.

See our [Contribution Guide](CONTRIBUTING.md)


## 🧪 Development

The frontend is written in 🦀 [Rust](https://rust-lang.org/) using the [Leptos](https://leptos.dev/) framework.


#### 🏃 Running

```shell
cargo leptos watch
```


#### 🔬 Testing

```shell
cargo leptos end-to-end
```

```shell
cargo leptos end-to-end --release
```

Cargo-leptos uses Playwright as the end-to-end test tool.  
Tests are located in end2end/tests directory.


#### 📦 Release

```shell
cargo leptos build --release
```

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:
```text
frontend
site/
```
Set the following environment variables (updating for your project as needed):
```text
LEPTOS_OUTPUT_NAME="frontend"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```


## 🚀 Release

To support multiple architectures, an own builder needs to be created.
```shell
docker buildx create --name multiarchitecturebuilder
docker buildx use multiarchitecturebuilder
docker buildx build --platform linux/arm64,linux/amd64 --tag photosnetwork/frontend:latest --push .
```



## 🏛️ License

```
Photos.network · A privacy first photo storage and sharing service for fediverse
Copyright (C) 2020 Photos network developers

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU Affero General Public License as
published by the Free Software Foundation, either version 3 of the
License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
