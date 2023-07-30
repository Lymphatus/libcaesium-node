# libcaesium-node

## Installing libcaesium

Installing libcaesium requires a [supported version of Node and Rust](https://github.com/neon-bindings/neon#platform-support). (Node 18 supported)

You can install the project with npm. In the project directory, run:

```sh
$ npm install
```

This fully installs the project, including installing any dependencies and running the build.

## Building libcaesium

If you have already installed the project and only want to run the build, run:

```sh
$ npm run build
```

This command uses the [cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact) utility to run the Rust build and copy the built library into `./build/caesium.node`.

## Exploring libcaesium
```javascript
const caesium = require('./build/caesium.node');

let parameters = {
    jpeg: {
        quality: 20
    },
    png: {
        quality: 20,
        force_zopfli: false
    },
    webp: {
        quality: 20
    },
    gif: {
        quality: 20
    },
    keep_metadata: true,
    optimize: false,
    width: 0,
    height: 0
}

const result = caesium.compress('input.jpg', 'output.jpg', parameters);

// { success: true, message: '' }
```

## Available Scripts

In the project directory, you can run:

### `npm install`

Installs the project, including running `npm run build`.

### `npm build`

Builds the Node addon (`index.node`) from source.

Additional [`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html) arguments may be passed to `npm build` and `npm build-*` commands. For example, to enable a [cargo feature](https://doc.rust-lang.org/cargo/reference/features.html):

```
npm run build -- --feature=beetle
```

#### `npm build-debug`

Alias for `npm build`.

#### `npm build-release`

Same as [`npm build`](#npm-build) but, builds the module with the [`release`](https://doc.rust-lang.org/cargo/reference/profiles.html#release) profile. Release builds will compile slower, but run faster.

## Learn More

To learn more about Neon, see the [Neon documentation](https://neon-bindings.com).

To learn more about Rust, see the [Rust documentation](https://www.rust-lang.org).

To learn more about Node, see the [Node documentation](https://nodejs.org).
