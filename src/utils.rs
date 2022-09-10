
pub fn first_zero_position(x: u8) -> u8
{
    let mut x = !x;  // 11110100b
    x = x ^ (x-1);  // 11110100 xor 11110011 = 00000111
    x = ((x >> 1) & 0b01010101)+(x & 0b01010101);  // 00 00 01 10
    x = ((x >> 2) & 0b00110011)+(x & 0b00110011);  // 0000 0011
    x = ((x >> 4) & 0b00001111)+(x & 0b00001111);  // 00000011 = 2

    x - 1
}