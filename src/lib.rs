mod listener;

pub use listener::ClipboardNotify;
#[cfg(test)]
mod tests {
    use crate::listener::*;
    #[test]
    fn it_works() {
        let clip = ClipboardNotify::new();
        clip.listen(|msg| {
            println!("{}", msg);
        }); 
    }
}
