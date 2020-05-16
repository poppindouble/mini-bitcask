pub fn u64_to_u8_array(x: u64) -> [u8; 8] {
    let b7 = ((x >> 56) & 0xff) as u8;
    let b6 = ((x >> 48) & 0xff) as u8;
    let b5 = ((x >> 40) & 0xff) as u8;
    let b4 = ((x >> 32) & 0xff) as u8;
    let b3 = ((x >> 24) & 0xff) as u8;
    let b2 = ((x >> 16) & 0xff) as u8;
    let b1 = ((x >> 8) & 0xff) as u8;
    let b0 = ((x >> 0) & 0xff) as u8;

    [b0, b1, b2, b3, b4, b5, b6, b7]
}

pub fn u8_array_to_u64(data: &[u8; 8]) -> u64 {
    (((data[0] as u64) << 0)
        + ((data[1] as u64) << 8)
        + ((data[2] as u64) << 16)
        + ((data[3] as u64) << 24)
        + ((data[4] as u64) << 32)
        + ((data[5] as u64) << 40)
        + ((data[6] as u64) << 48)
        + ((data[7] as u64) << 56))
        .into()
}
