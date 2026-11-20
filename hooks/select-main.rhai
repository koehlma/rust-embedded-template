// The other entry points are already ignored by `cargo-generate.toml`.
let main = if variable::get("wifi") {
    "src/main.embassy-wifi.rs"
} else if variable::get("embassy") {
    "src/main.embassy.rs"
} else {
    "src/main.rp-hal.rs"
};
file::rename(main, "src/main.rs");

// The hook scripts are ignored, but their directory remains.
file::delete("hooks");
