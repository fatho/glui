#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Id(u64);

impl Id {
    /// *The* invalid ID. Used where it is necessary to explicitly
    /// state that no existing widget is referenced.
    #[inline(always)]
    pub const fn invalid() -> Id {
        Id(0xFFFF_FFFF_FFFF_FFFF)
    }
}

/// Generate a unique widget ID based on the source location.
#[macro_export]
macro_rules! mk_id {
    () => {
        $crate::core::make_id(line!(), column!())
    };
}

macro_rules! fnv_byte {
    ($hash: ident, $data: expr) => {
        $hash.wrapping_mul(0x100000001b3) ^ ($data as u8) as u64
    };
}

macro_rules! fnv_u32 {
    ($hash: ident, $data: expr) => {
        {
            let data = $data as u32;
            let mut hash = $hash;
            hash = fnv_byte!(hash, data >> 0);
            hash = fnv_byte!(hash, data >> 8);
            hash = fnv_byte!(hash, data >> 16);
            hash = fnv_byte!(hash, data >> 24);
            hash
        }
    };
}

/// Build a widget ID from line and column where the generator macro is used.
pub const fn make_id(line: u32, column: u32) -> Id {
    let mut fnv = 0xcbf29ce484222325u64;

    fnv = fnv_u32!(fnv, line);
    fnv = fnv_u32!(fnv, column);

    Id(fnv)
}