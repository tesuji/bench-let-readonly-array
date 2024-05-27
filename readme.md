bench targets
* small size <= 16 bytes: llvm inlines as single mov: 1, 2, 4, 8, 16
* medium <= 256 bytes: llvm inlines memcpy as multiple move: 32, 64, 128, 256
* large > 256 bytes: llvm uses memcpy: 512, 1024, 2048, 4096

Literal: 
* integers, static string
* String?

Bench this code
```rust
pub fn indexing(x: usize) -> u8 {
	let array: [u8; N] = ?;
	array[x]
}
```

Need: codegen? to
* generate array
* a simple [{int}; N]: memcpy? zero or non-zero
* random items: [1, 2, ..., rand()]
* x:
  + is inside: choose happy path
  + out of bound => panic ?

---

output:
rustc -Vv
bench machine: cpu, os?
plot .dot/.svg
