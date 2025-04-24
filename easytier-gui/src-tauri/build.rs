fn main() {
    #[cfg(all(windows, target_env = "msvc", not(target_arch = "aarch64")))]
    thunk::thunk();

    tauri_build::build();
}
