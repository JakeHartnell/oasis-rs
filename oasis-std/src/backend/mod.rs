cfg_if::cfg_if! {
    if #[cfg(all(target_arch = "wasm32", target_os = "wasi"))] {
        mod wasi;
        use wasi as imp;
    } else {
        mod ext;
        use ext as imp;
    }
}

pub use imp::{
    aad, address, balance, code, emit, err, input, payer, read, ret, sender, transact, value, write,
};

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// Unknown error occured
    Unknown,

    /// Not enough funds to pay for transaction
    InsufficientFunds,

    /// Invalid input provided to transaction
    InvalidInput,

    /// No callable code at destination address
    InvalidCallee,

    /// Transaction failed with status code and payload
    Execution { payload: Vec<u8> },
}
