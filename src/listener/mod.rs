cfg_if::cfg_if! {
    if #[cfg(target_os = "windows")] {
        mod windows;
        pub use windows::ClipboardNotify;
    } else {
        mod x11;
        pub use x11::ClipboardNotify;

    }
}
