use pulldown_cmark::Options;

pub fn markdown_options() -> Options {
    Options::ENABLE_TABLES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_YAML_STYLE_METADATA_BLOCKS
        | Options::ENABLE_MATH
        | Options::ENABLE_WIKILINKS
}

pub fn line_ending(source: &str) -> &'static str {
    match source.find(['\r', '\n']) {
        Some(at) if source[at..].starts_with("\r\n") => "\r\n",
        Some(at) if source.as_bytes()[at] == b'\r' => "\r",
        _ => "\n",
    }
}

#[cfg(target_arch = "wasm32")]
mod abi {
    use std::cell::RefCell;

    const MAX_TEXT: usize = 4 * 1024 * 1024;

    thread_local! {
        static INPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
        static OUTPUT: RefCell<String> = const { RefCell::new(String::new()) };
    }

    pub fn allocate(length: u32) -> *mut u8 {
        if length as usize > MAX_TEXT {
            return std::ptr::null_mut();
        }
        INPUT.with_borrow_mut(|input| {
            input.resize(length as usize, 0);
            input.as_mut_ptr()
        })
    }

    pub fn run(pointer: *const u8, length: u32, transform: fn(&str) -> Option<String>) -> u64 {
        INPUT.with_borrow(|input| {
            // 只读取由 alloc 准备的缓冲区，命令无需解引用外部传入的地址。
            if pointer != input.as_ptr() || length as usize != input.len() {
                return 0;
            }
            let Some(modified) = str::from_utf8(input).ok().and_then(transform) else {
                return pack(input.as_ptr(), input.len());
            };
            OUTPUT.with_borrow_mut(|output| {
                *output = modified;
                pack(output.as_ptr(), output.len())
            })
        })
    }

    fn pack(pointer: *const u8, length: usize) -> u64 {
        ((pointer as usize as u64) << 32) | length as u64
    }
}

#[cfg(target_arch = "wasm32")]
pub use abi::{allocate, run};

#[macro_export]
macro_rules! export_commands {
    ($($export:ident => $transform:path),+ $(,)?) => {
        fn main() {}

        #[cfg(target_arch = "wasm32")]
        #[unsafe(no_mangle)]
        pub extern "C" fn alloc(length: u32) -> *mut u8 {
            $crate::allocate(length)
        }

        $(
            #[cfg(target_arch = "wasm32")]
            #[unsafe(no_mangle)]
            pub extern "C" fn $export(pointer: *const u8, length: u32) -> u64 {
                $crate::run(pointer, length, $transform)
            }
        )+
    };
}
