# Welcome to Photos.network <!-- omit in toc -->

This is a **FOSS** (free and open-source software) and lives from contributions of the community.

There are many ways to contribute:

 * 📣 Spread the project or its apps to the world
 * ✍️ Writing tutorials and blog posts
 * 📝 Create or update the documentation
 * 🐛 Submit bug reports
 * 💡 Adding ideas and feature requests to Discussions
 * 👩‍🎨 Create designs or UX flows
 * 🧑‍💻 Contribute code or review PRs



## 📜 Ground Rules

A community like this should be **open**, **considerate** and **respectful**.

Behaviours that reinforce these values contribute to a positive environment, and include:

 * **Being open**. Members of the community are open to collaboration.
 * **Focusing on what is best for the community**. We're respectful of the processes set forth in the community, and we work within them.
 * **Acknowledging time and effort**. We're respectful and thoughtful when addressing the efforts of others, keeping in mind that often times the labor was completed simply for the good of the community.
 * **Being respectful of differing viewpoints and experiences**. We're receptive to constructive comments and criticism, as the experiences and skill sets of other members contribute to the whole of our efforts.
 * **Showing empathy towards other community members**. We're attentive in our communications, whether in person or online, and we're tactful when approaching differing views.
 * **Being considerate**. Members of the community are considerate of their peers.
 * **Being respectful**. We're respectful of others, their positions, their skills, their commitments, and their efforts.
 * **Gracefully accepting constructive criticism**. When we disagree, we are courteous in raising our issues.
 * **Using welcoming and inclusive language**. We're accepting of all who wish to take part in our activities, fostering an environment where anyone can participate and everyone can make a difference.



## 🧑‍💻 Code Contribution

To contribute code to the repository, you don't need any permissions.
First start by forking the repository, clone and checkout your clone and start coding.
When you're happy with your changes, create Atomic commits on a **new feature branch** and push it to ***your*** fork.

Atomic commits will make it easier to track down regressions. Also, it enables the ability to cherry-pick or revert a change if needed.

1. Fork it (https://github.com/photos-network/core/fork)
2. Create a new feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request



## 🐛 How to report a bug

> If you find a security vulnerability, do NOT open an issue. Email [security@photos.network](mailto:security@photos.network) instead. See [SECURITY.md](./SECURITY.md) for details.

1. Open the [issues tab](https://github.com/photos-network/core/issues) on github
2. Click on [New issue](https://github.com/photos-network/core/issues/new/choose)
3. Choose the bug report 🐛 template and fill out all required fields



## 💡 How to suggest a feature or enhancement

Check [open issues](https://github.com/photos-network/core/issues) for a list of proposed features.

If your suggestion can not be found already, see if it is already covered by our [Roadmap](https://github.com/photos-network/core/#roadmap).



## 📟 Communication

To get in touch with the community or write use on Mastodon: [@photos@mastodon.cloud](https://mastodon.cloud/@photos).



## 💾 Technology

The project is written in [Rust](https://rust-lang.org/) 

Underneath it is using these frameworks:

* [tokio](https://github.com/tokio-rs/tokio) - an asynchronous runtime
* [tower](https://github.com/tower-rs/tower) - for networking
* [axum](https://github.com/tokio-rs/axum) - as web framework
* [abi_stable](https://github.com/rodrimati1992/abi_stable_crates) - FFI for dynamic library loading


## 🧪 Development

The frontend is written in 🦀 [Rust](https://rust-lang.org/) using the 🚀 [Leptos](https://leptos.dev/) framework.

It requires to install [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) and the [tailwind css executable](https://github.com/tailwindlabs/tailwindcss/releases)

#### 🏃 Running

First the tailwind css needs to be created if modified
```shell
tailwindcss -i ./input.css -o ./style/output.css --watch
```

Next leptos needs to compile the application
```shell
cargo leptos watch
```

Open any browser at [http://127.0.0.1:7778](http://127.0.0.1:7778)



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


