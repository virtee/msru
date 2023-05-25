# msru

`msru` provides a Rust-friendly interface for x86_64 CPU MSRs
(Model Specific Registers). It allows you to read and write to MSRs
via a specified CPU msr device file (`/dev/cpu/{cpu_number}/msr`).

## Installation

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
msru = "0.2.0"
```

## Usage

```rust
use msru::Msr;

// X86_64 SYSCFG MSR
let msr: Msr = Msr::new(0xC0010010, 0)?;

let raw_value: u64 = msr.read()?;

// ...
```