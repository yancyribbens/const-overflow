### Running in release mode will silently overflow in const context.

*Compile with release mode optimizations:*
```
rustc -C debuginfo=0 -C opt-level=3 ./main.rs
./main 4
```

### Running in debug mode (no optimizations) will panic instead of silently overflow.

*Compile with release mode optimizations:*
```
rustc ./main.rs
./main 4
```
