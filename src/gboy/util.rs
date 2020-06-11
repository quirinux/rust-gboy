//use std::mem;


pub fn split_bytes(value: u16) -> (u8, u8) {
    let v = value.to_be_bytes();
    (v[0], v[1])
}

pub fn join_bytes(a: u8, b: u8) -> u16 {
    u16::from_be_bytes([a, b])
}

// pub fn u8_to_i8(from: u8) -> i8 {
//     unsafe {        
//         mem::transmute::<u8, i8>(from)
//     }
// }

pub fn half_carry_occured(a: u8) -> bool {
    // let half_carry_mask: u8 = 0b0001_0000;
    // a & half_carry_mask == half_carry_mask
    a > 0x0F
}
