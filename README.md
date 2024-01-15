
# open-stylus

Abstracted OpenZeppelin smart contracts written in Rust for Arbitrum Stylus.

## Project description

open-stylus is a library of smart contracts written in Rust for Arbitrum Stylus. It aims to abstract some parts of the OpenZeppelin code and adapt it to Rust for Stylus. 

Arbitrum Stylus is a new technology that allows developers to write EVM-compatible smart contracts in many different programming languages, such as Rust, C, and C++, by using WebAssembly (WASM) as a second, coequal virtual machine. This enables faster, cheaper, and more efficient computation and memory usage on Arbitrum chains.

## Installation and usage

To use open-stylus, you need to have the following prerequisites installed:

- Rust: The programming language used to write the smart contracts. You can install it from [here](https://www.freecodecamp.org/news/how-to-write-and-deploy-your-first-smart-contract-341d5e2ffb35/).
- Cargo: The Rust package manager and build tool. You can install it from [here](https://www.freecodecamp.org/news/how-to-write-a-good-readme-file/).
- Stylus: The Arbitrum toolchain for compiling and deploying smart contracts. You can install it from [here](https://dev.to/github/how-to-create-the-perfect-readme-for-your-open-source-project-1k69).

To install open-stylus, you can clone this repository and run the following command in the root directory:

```bash
cargo install --path .

```

This will install the open-stylus library as a dependency in your project.

To use open-stylus, you can import the modules you need in your Rust code. For example, to use the ERC20 module, you can write:

Rust

```rust
use open_stylus::acess::Ownable;

```

Then, you can create and interact with ERC20 tokens using the methods provided by the module. For example, to create a new token with a name, symbol, and initial supply, you can write:

Rust


## Contributing

open-stylus is an open source project and welcomes contributions from anyone who is interested in improving it. If you want to contribute to open-stylus, you can follow these steps:

-   Fork this repository and clone it to your local machine.
-   Create a new branch for your feature or bug fix.
-   Format your code using  `cargo fmt`  and lint it using  `cargo clippy`.
-   Commit your changes and push them to your fork.
-   Create a pull request to the main branch of this repository.

Please make sure to follow the code style, testing, and documentation guidelines of this project. You can also check the issues page for any open tasks or suggestions.

## License

open-stylus is licensed under the MIT License.

```

I hope this template helps you write a good README file for your open-stylus project. If you have any questions or feedback, please let me know. 😊
```

<div style="color: red; background-color: #ffe5e5; padding: 10px; border-radius: 5px;">
  <strong>Warning:</strong> This code is unaudited. Use it at your own risk.
</div>
