use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

static BACKTRACE: Lazy<Arc<Mutex<Vec<BtFrame>>>> = Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BtFrame {
    /// The symbol name
    pub symbol: &'static str,

    /// The file path
    pub file: &'static str,

    /// module_path!() + file!()
    pub module_path: &'static str,

    /// line!()
    pub line: u32,
}

pub fn push_frame(
    symbol: &'static str,
    module_path: &'static str,
    file: &'static str,
    line: u32,
) {
    let frame = BtFrame {
        module_path,
        file,
        symbol,
        line,
    };

    BACKTRACE.lock().unwrap().push(frame);
}

pub fn pop_frame() {
    BACKTRACE.lock().unwrap().pop();
}

pub fn get_backtrace() -> Vec<BtFrame> {
    BACKTRACE.lock().unwrap().clone()
}
