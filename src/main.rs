use std::time::{SystemTime, UNIX_EPOCH};

/// Seconds from Unix epoch to 2025-01-01 00:00:00 UTC.
const CUSTOM_EPOCH: i64 = 1_735_689_600;
const URL_SAFE_B64: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

fn encode_five_chars(ts: u32) -> String {
    // Take the top-to-bottom 5×6-bit slices of the 30-bit value.
    let mut out = String::with_capacity(5);
    let shifts = [24, 18, 12, 6, 0];
    for &shift in &shifts {
        out.push(URL_SAFE_B64[((ts >> shift) & 0x3F) as usize] as char);
    }
    out
}

fn main() {
    // Seconds since Unix epoch right now.
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Clock went backwards")
        .as_secs() as i64;

    // Seconds since custom epoch, clamped at 0 (handles times before 2025).
    let diff = (now - CUSTOM_EPOCH).max(0) as u32; // fits in 30 bits until 2059

    // Encode and print with the leading caret.
    println!("{}", encode_five_chars(diff));
} 