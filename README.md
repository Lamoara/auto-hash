# auto-hash

Small CLI wrapper around hashcat. Allows specifying patterns either via command line or a file.

## Usage

```bash
cargo run --release -- \ \
  --input-file hashes.txt \
  --output results.txt \
  --alphabet abcdef \
  --min 1 --max 3 \
  --patterns ?d?d \
  --t 0 
```

### Pattern file

Patterns can also be provided through a file with `--pattern-file`. Each line of the file is treated as a pattern and appended to any patterns passed directly by `--patterns`.

```bash
cargo run --release -- --input-file hashes.txt --output results.txt \
  --alphabet abcdef --min 1 --max 3 \
  --pattern-file patterns.txt --t 0
```
