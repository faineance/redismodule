# redismodule [![Crates.io](https://img.shields.io/crates/v/redismodule.svg?style=flat-square)](https://crates.io/crates/redismodule)
Write Redis modules in Rust.

[Documentation](https://docs.rs/redismodule)

## Running the example

'''bash
$ cd example && cargo build
$ redis-server --loadmodule ./target/debug/libexample.so
$ echo "array hello world 123" | redis-cli  
1) (integer) 4
2) "array"
3) "hello"
4) "world"
5) "123"

'''
