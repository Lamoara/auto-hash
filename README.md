# Auto Hash

Utility to automate Hashcat tasks from Rust.

## Usage

Run the binary with the desired options:

```bash
auto-hash \
  --input-file <path> \
  --output <result file> \
  --min <length> \
  --max <length> \
  --alphabet <chars> \
  --patterns <patterns>... \
  --t <hash types>... \
  [--attack-mode <mode>]
```

### Parameters

- `--input-file` Path to the hashes to crack.
- `--output` File where results are written.
- `--min` Minimum length when expanding `?x` patterns.
- `--max` Maximum length when expanding `?x` patterns.
- `--alphabet` Characters used for the custom charset `?1`.
- `--patterns` Wordlist of patterns to try. Use `?x` to expand based on `min` and `max`.
- `--t` Hash types passed to Hashcat.
- `--attack-mode` Hashcat attack mode (default: `3`).

The program prints each attempt and displays the contents of the output file when finished.
