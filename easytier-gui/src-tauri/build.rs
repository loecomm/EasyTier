fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default() == "windows"
        && std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "msvc"
        && std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() == "x86"
    {
        thunk::thunk();
    }

    tauri_build::build();
}
