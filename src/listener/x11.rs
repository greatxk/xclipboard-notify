
use x11_clipboard::Clipboard;

pub struct ClipboardNotify {
}

impl ClipboardNotify {
    pub fn new() -> ClipboardNotify {
        ClipboardNotify{}
    }

    pub fn listen<F>(&self, handle: F)
    where
        F: Fn(String),
    {
        let clipboard = Clipboard::new().unwrap();
        let mut last = String::new();
        loop {
            if let Ok(curr) = clipboard.load_wait(
                clipboard.getter.atoms.clipboard,
                clipboard.getter.atoms.utf8_string,
                clipboard.getter.atoms.property,
            ) {
                let curr = String::from_utf8_lossy(&curr);
                let curr = curr.trim_matches('\u{0}').trim();
                if !curr.is_empty() && last != curr {
                    last = curr.to_owned();
                }
                handle(curr);
            }
        }

    }
}
