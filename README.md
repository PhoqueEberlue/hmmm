# HMMM - Hypercube Matrix-Matrix Multiplication

Implementation of a Systolic algorithm for Matrix-Matrix Multiplication with Hypercube Architecture.

## Structure of the project

Two different Implementations have been realised for this project:
- Pure Rust with a CLI display in folder `cli-hmmm/`
- WebAssembly + Rust displaying the Hypercube via [Plotly](https://plotly.com/) in folder `wasm-hmmm/`
 
## Running pure Rust project

Install [rust](https://www.rust-lang.org/tools/install) and run the following command in `cli-hmmm\`:

```shell
cargo run

         A               B               C
      0 0 0 0         0 1 2 0         0 0 0 0 
    3 3 3 3         0 1 2 0         0 3 6 0 
  2 2 2 2         0 1 2 0         0 2 4 0 
0 0 0 0         0 1 2 0         0 0 0 0 
      0 0 0 0         1 0 1 2         0 0 0 0 
    1 1 1 1         1 0 1 2         1 3 7 2 
  0 0 0 0         1 0 1 2         0 2 4 0 
2 2 2 2         1 0 1 2         2 0 2 4 
      2 2 2 2         0 3 2 0         0 6 4 0 
    1 1 1 1         0 3 2 0         1 6 9 2 
  0 0 0 0         0 3 2 0         0 2 4 0 
0 0 0 0         0 3 2 0         2 0 2 4 
      1 1 1 1         1 2 2 0         1 8 6 0 
    0 0 0 0         1 2 2 0         1 6 9 2 
  1 1 1 1         1 2 2 0         1 4 6 0 
0 0 0 0         1 2 2 0         2 0 2 4 
```

## Running WebAssembly app

Follow the [setup](https://rustwasm.github.io/docs/book/game-of-life/setup.html) for compiling Rust WebAssembly apps.

Then in `wasm-hmmm/` build the project with
```shell
wasm-pack build
```

Go in `wasm-hmmm/www/` and install npm dependencies
```shell
npm install
```
or
```shell
yarn
```

And launch the server
```shell
npm run start
```
or
```shell
yarn start
```

Yarn worked better for me in term of dependency resolving, but it's up to you, anyways I'm not a Webdev idk what I'm doing 💀

Here is an example of what it looks like:

![example-wasm](https://github.com/PhoqueEberlue/game-of-rust/blob/main/example-wasm.png)



