### Structure

+ `rust-foo` - Rust library
+ `c-bindings` - C bindings
+ `cpp-bindings` - C++ bindings
+ `ruby-bindings` - Ruby bindings

### Rust version

To run Rust tests with Rust primitives ("native" mode):

```sh
$ BUILD_LANG=Rust make rust-foo/test
```

### C version

To run Rust tests with C primitives ("external" mode):

```sh
$ BUILD_LANG=C make rust-foo/test
```

To run C tests with Rust library compiled with C primitives (an actual usage example):

```
$ BUILD_LANG=C make c-bindings/test
```

### C++ version

To run Rust tests with C++ primitives ("external" mode):

```sh
$ BUILD_LANG=C++ make rust-foo/test
```

To run C++ tests with Rust library compiled with C++ primitives (an actual usage example):

```
$ BUILD_LANG=C++ make cpp-bindings/test
```

### Ruby version

It is impossible to run Rust tests with Ruby primitives.

To run Ruby tests with Rust library compiled with Ruby primitives (an actual usage example):

```sh
$ BUILD_LANG=Ruby make ruby-bindings/test
```

On Linux you also need to set environment variable `DYLIB=so`, otherwise make scripts expect dynamic library to have a `.bundle` extension.

### Troubleshooting

If something doesn't work check CI configs in `.github/workflows` directory
