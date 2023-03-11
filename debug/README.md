# 🔤 Debug

```rs
// Struct cannot be printed either with `fmt::Display` or with `fmt::Debug`.
struct UnPrintable(i32);

// The `derive` attribute automatically creates the implementation
// required to make this `struct` printable with `fmt::Debug`.
#[derive(Debug)]
struct DebugPrintable(i32);
```

```rs
// Now we can print it!
println!("{:?}", DebugPrintable(3));
```

https://doc.rust-lang.org/rust-by-example/hello/print/print_debug.html
