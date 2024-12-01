# a simple template for rust
```
➜  hello_world_rust git:(main) cargo test
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.14s
     Running unittests src/main.rs (target/debug/deps/hello_world_rust-0248ba7bde07160e)

running 2 tests
test tests::test_greet ... ok
test tests::test_greet_empty_name ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

➜  hello_world_rust git:(main) cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/hello_world_rust`
Hello, Rustacean!
```
