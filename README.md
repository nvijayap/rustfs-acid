# rustfs-acid
RustFS (Distributed S3 Compliant Storage) - ACID (Atomicity, Consistency, Isolation, and Durability)

## Approach / Mechanism
Using channels to ensure ACID compliance by way of safe concurrency for bucket operations

## NOTES
  * The AWS Command-Line Interface (CLI) is not inherently thread-safe, meaning that using a single instance of the CLI in multiple threads can lead to unexpected behavior. It is recommended to create separate instances for each thread to ensure safe operation.

  * So, this repo addresses that problem by using safe concurrency approach in Rust

## References
  1. [RustFS](https://rustfs.com/) - RUST-NATIVE OBJECT STORAGE - High-Performance, S3-Compatible Object Storage for AI Data Centers
  2. [Rust](https://rust-lang.org/) - A language empowering everyone to build reliable and efficient software
