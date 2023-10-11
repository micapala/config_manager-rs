<div align="center">
<h1 align="center">
<img src="https://raw.githubusercontent.com/PKief/vscode-material-icon-theme/ec559a9f6bfd399b82bb44393651661b08aaf7ba/icons/folder-markdown-open.svg" width="100" />
<br>config_manager-rs</h1>
<h3>◦ Effortless config management, powered by config_manager-rs</h3>
<h3>◦ Developed with the software and tools below.</h3>

<p align="center">
<img src="https://img.shields.io/badge/Rust-000000.svg?style&logo=Rust&logoColor=white" alt="Rust" />
<img src="https://img.shields.io/badge/Hyper-000000.svg?style&logo=Hyper&logoColor=white" alt="Hyper" />
</p>
<img src="https://img.shields.io/github/license/michi0987/config_manager-rs?style&color=5D6D7E" alt="GitHub license" />
<img src="https://img.shields.io/github/last-commit/michi0987/config_manager-rs?style&color=5D6D7E" alt="git-last-commit" />
<img src="https://img.shields.io/github/commit-activity/m/michi0987/config_manager-rs?style&color=5D6D7E" alt="GitHub commit activity" />
<img src="https://img.shields.io/github/languages/top/michi0987/config_manager-rs?style&color=5D6D7E" alt="GitHub top language" />
</div>

---

## 📖 Table of Contents
- [📖 Table of Contents](#-table-of-contents)
- [📍 Overview](#-overview)
- [📦 Features](#-features)
- [📂 Repository Structure](#-repository-structure)
- [⚙️ Modules](#modules)
- [🚀 Getting Started](#-getting-started)
    - [🔧 Installation](#-installation)
    - [🤖 Running config_manager-rs](#-running-config_manager-rs)
    - [🧪 Tests](#-tests)
- [🛣 Roadmap](#-roadmap)
- [🤝 Contributing](#-contributing)
- [📄 License](#-license)
- [👏 Acknowledgments](#-acknowledgments)

---


## 📍 Overview

The project is a configuration management system written in Rust. It provides a REST API and a gRPC service for managing configuration groups. The system allows users to perform CRUD operations on configuration groups, retrieve specific configuration parameters, and provides a file watcher functionality to reload the configuration when changes are detected. Its value proposition lies in the ability to easily manage and update configurations in a scalable and efficient manner.

---

## 📦 Features

|    | Feature            | Description                                                                                                        |
|----|--------------------|--------------------------------------------------------------------------------------------------------------------|
| ⚙️ | **Architecture**   | The codebase follows a modular and component-based architecture. It uses a trait-based design pattern to define the behavior of different components. The architecture enables easy extensibility and reusability of code. |
| 🔗 | **Dependencies**   | The codebase relies on several external libraries, including serde for serialization/deserialization, clap for command-line argument parsing, and log for logging purposes. These dependencies are well-managed using Cargo, the Rust package manager. |
| 🧩 | **Modularity**     | The codebase is highly modular, with clear separation between different concerns. It defines reusable traits and provides implementations for different components, allowing easy customization and extension. |
| 🔀 | **Version Control**| The project uses Git for version control. The repository follows best practices, including commits being made in small, logical increments, and the use of branches for new features or bug fixes. |
| 🔌 | **Integrations**   | The codebase provides integrations with external systems and services, mainly through the use of configuration files. It allows users to specify the configurations of different components and integrate them seamlessly. |
| 📶 | **Scalability**    | The codebase is designed to be scalable and adaptable to handle growing requirements. It provides abstractions and modular components that can be easily extended or replaced as needed, enabling the system to scale with minimal impact on existing functionality. |

---


## 📂 Repository Structure

```sh
└── config_manager-rs/
    ├── .gitignore
    ├── Cargo.lock
    ├── Cargo.toml
    ├── build.rs
    ├── proto/
    │   └── configuration.proto
    └── src/
        ├── api/
        ├── config/
        ├── file_watcher/
        ├── grpc/
        ├── main.rs
        └── multiplex_service/
```


---

## ⚙️ Modules

<details closed><summary>Root</summary>

| File                                                                              | Summary                                                                                                                                                                                                                    |
| ---                                                                               | ---                                                                                                                                                                                                                        |
| [Cargo.toml](https://github.com/michi0987/config_manager-rs/blob/main/Cargo.toml) | The code contains dependencies and their versions for a Rust package called "config_manager". These dependencies include libraries for web development, command-line parsing, logging, asynchronous programming, and more. |
| [build.rs](https://github.com/michi0987/config_manager-rs/blob/main/build.rs)     | This code is a build script that uses the Tonic library to generate Rust code from a protocol buffer file. It configures the output path for the generated code and compiles it.                                           |

</details>

<details closed><summary>Proto</summary>

| File                                                                                                      | Summary                                                                                                                                                                                                                                                                              |
| ---                                                                                                       | ---                                                                                                                                                                                                                                                                                  |
| [configuration.proto](https://github.com/michi0987/config_manager-rs/blob/main/proto/configuration.proto) | The code defines a protocol buffer structure for managing configuration groups. It includes CRUD operations like creating, reading, updating, and deleting groups. The service provides methods to perform these operations, along with corresponding request and response messages. |

</details>

<details closed><summary>Src</summary>

| File                                                                            | Summary                                                                                                                                                                                                                                                                                   |
| ---                                                                             | ---                                                                                                                                                                                                                                                                                       |
| [main.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/main.rs) | This code is for a configuration management server. It includes functionality for handling REST API requests, serving Swagger UI documentation, and a gRPC service for managing configurations. It also includes a file watcher that reloads the configuration when changes are detected. |

</details>

<details closed><summary>Api</summary>

| File                                                                                                  | Summary                                                                                                                                                                                                                                                                                                                                                                                          |
| ---                                                                                                   | ---                                                                                                                                                                                                                                                                                                                                                                                              |
| [documentation.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/api/documentation.rs) | The code in `documentation.rs` provides an OpenAPI documentation generator for the API routes and tags defined in the `rest` module. It creates documentation for API paths related to listing groups, getting a specific group, and retrieving a config parameter. The documentation includes a tag for the configuration manager API.                                                          |
| [rest.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/api/rest.rs)                   | Definition of endpoints handlers. |
| [mod.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/api/mod.rs)                     | Module file for REST API.                                                                                                                                                                                |

</details>

<details closed><summary>Grpc</summary>

| File                                                                                 | Summary                                                                                                                                                                                                                                                                                                                                                       |
| ---                                                                                  | ---                                                                                                                                                                                                                                                                                                                                                           |
| [grpc.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/grpc/grpc.rs) | The code provides an implementation for a gRPC server's ConfigurationManager service. It allows creating, reading, updating, and deleting groups, as well as retrieving all groups. The shared_state field is used for synchronization.                                                                                                                       |
| [mod.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/grpc/mod.rs)   | The file "mod.rs" in the "grpc" directory contains a Rust module called "grpc" that provides core functionalities for working with gRPC, a high-performance remote procedure call (RPC) framework. It likely includes modules and functions for handling communication, serialization, and deserialization between client and server using the gRPC protocol. |

</details>

<details closed><summary>Config</summary>

| File                                                                                                     | Summary                                                                                                                                                                                                                                                                                                                                                                    |
| ---                                                                                                      | ---                                                                                                                                                                                                                                                                                                                                                                        |
| [configuration.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/config/configuration.rs) | The `ConfigurationGroups` struct represents a collection of configuration groups, where each group is identified by a unique name. It provides functions to add, retrieve, and remove groups from the collection. It also implements serialization and deserialization functionality using serde_yaml. The test cases validate the deserialization behavior of the struct. |
| [group.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/config/group.rs)                 | The `group.rs` code file contains the implementation of the `ConfigurationGroup` struct, which represents a group of configuration entries. It provides methods to add and remove entries, process group operations, and convert the group into a protobuf message. The tests ensure the correctness of the implementation.                                                |
| [mod.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/config/mod.rs)                     | The code consists of three modules: configuration, manager, and group. It also exports all items from the manager module. The purpose and specifics of these modules are not explicitly mentioned in the given code snippet.                                                                                                                                               |
| [manager.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/config/manager.rs)             | The code in `manager.rs` defines a `ConfigurationManager` struct that manages configuration data. It can read configuration from a YAML file, save it to a file, and reload it. The struct uses a `ConfigurationGroups` struct to store the configuration data. It supports both YAML and JSON backend, but only YAML backend is implemented.                              |

</details>

<details closed><summary>Multiplex_service</summary>

| File                                                                                                                        | Summary                                                                                                                                                                                                                                                                                                       |
| ---                                                                                                                         | ---                                                                                                                                                                                                                                                                                                           |
| [mod.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/multiplex_service/mod.rs)                             | Module file.                                                                                                                      |
| [multiplex_service.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/multiplex_service/multiplex_service.rs) | The code defines a `MultiplexService` that acts as a router for REST and gRPC requests. It determines the type of request and calls the appropriate service. It ensures readiness of the services before handling requests. The code also includes a helper function to check if a request is a gRPC request. |

</details>

<details closed><summary>File_watcher</summary>

| File                                                                                                         | Summary                                                                                                                                                                                                                                                                                                                                |
| ---                                                                                                          | ---                                                                                                                                                                                                                                                                                                                                    |
| [file_watcher.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/file_watcher/file_watcher.rs) | The `FileWatcher` struct encapsulates the functionalities of watching for file events in a specified path. It uses the `notify` crate and asynchronous channels to receive file events. The `new` method initializes the watcher and starts watching the specified path. The `watch` method waits for and returns the next file event. |
| [mod.rs](https://github.com/michi0987/config_manager-rs/blob/main/src/file_watcher/mod.rs)                   | The code in `mod.rs` defines the file_watcher module, which includes the `file_watcher` submodule. This submodule likely contains the core functionalities for monitoring and detecting changes in files.                                                                                                                              |

</details>

---

## 🚀 Getting Started

***Dependencies***

Please ensure you have the following dependencies installed on your system:

`- ℹ️ Rust`

`- ℹ️ Protobuf compiler`

### 🔧 Installation

1. Clone the config_manager-rs repository:
```sh
git clone https://github.com/michi0987/config_manager-rs
```

2. Change to the project directory:
```sh
cd config_manager-rs
```

3. Install the dependencies:
```sh
cargo build
```

### 🤖 Running config_manager-rs

```sh
cargo run
```

### 🧪 Tests
```sh
cargo test
```

---


## 🛣 Roadmap

> - [X] `ℹ️  Registration of clients so they get notified when changes to configuration are made X`
> - [ ] `ℹ️ ...`

---

## 📄 License

This project is licensed under the `ℹ️  MIT` License. See the [LICENSE](LICENSE) file for additional info.

---

## 👏 Acknowledgments

`- ℹ️ None for now`

[↑ Return](#Top)

---
