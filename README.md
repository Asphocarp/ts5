# ts5 - Time Stamp in 5 Characters

A minimalist Rust CLI that generates 5-character, URL-safe Base64 timestamps.

⚠️ **Important Limitation**: This toy timestamp format will only work correctly until February 7, 2059. After this date, the 30-bit counter will overflow and become ts6.

## How It Works

The program encodes the time difference in seconds between now and a custom epoch (2025-01-01 00:00:00 UTC) as a 5-character, URL-safe Base64 string.

- Uses a custom epoch (1735689600 seconds since Unix epoch)
- Fits in 30 bits until Feb 7, 2059, keeping the output at exactly 5 characters
- Uses URL-safe Base64 alphabet: `A-Za-z0-9-_`

## Building and Running

```bash
# Build the release version
cargo build --release

# Run the program
cargo run --release
# Output will be something like: AoV_L
```

## Implementation Details

- The program works by taking the seconds since the custom epoch
- Values are clamped at 0 to handle times before 2025
- Each 5-character timestamp represents a unique moment in time
- The encoding extracts 6-bit windows at bit positions 24, 18, 12, 6, 0 and maps them directly to the Base64 alphabet 