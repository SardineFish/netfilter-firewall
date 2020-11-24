use alloc::format;

pub fn ipv4_addr(addr: u32) -> alloc::string::String {
    let bytes = addr.to_be_bytes();
    format!("{}.{}.{}.{}", bytes[3], bytes[2], bytes[1], bytes[0])
}