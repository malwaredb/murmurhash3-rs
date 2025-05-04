use std::ptr;

fn fmix32(mut h: u32) -> u32 {
    h ^= h >> 16;
    h = h.wrapping_mul(0x85ebca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2ae35);
    h ^= h >> 16;

    h
}

fn get_32_block(bytes: &[u8], index: usize) -> u32 {
    unsafe {
        let p = bytes.as_ptr().add(index * 4);
        ptr::read_unaligned(p.cast())
    }
}

pub fn murmurhash3_x86_32(bytes: &[u8], seed: u32) -> u32 {
    let c1 = 0xcc9e2d51u32;
    let c2 = 0x1b873593u32;
    let read_size = 4;
    let len = bytes.len() as u32;
    let block_count = len / read_size;

    let mut h1 = seed;

    for i in 0..block_count as usize {
        let mut k1 = get_32_block(bytes, i);

        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(c2);

        h1 ^= k1;
        h1 = h1.rotate_left(13);
        h1 = h1.wrapping_mul(5);
        h1 = h1.wrapping_add(0xe6546b64)
    }
    let mut k1 = 0u32;

    if len & 3 == 3 {
        k1 ^= (bytes[(block_count * read_size) as usize + 2] as u32) << 16;
    }
    if len & 3 >= 2 {
        k1 ^= (bytes[(block_count * read_size) as usize + 1] as u32) << 8;
    }
    if len & 3 >= 1 {
        k1 ^= bytes[(block_count * read_size) as usize] as u32;
        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
    }

    h1 ^= bytes.len() as u32;
    h1 = fmix32(h1);

    h1
}

#[cfg(test)]
mod test {
    use super::murmurhash3_x86_32;

    #[test]
    fn test_empty_string() {
        assert_eq!(murmurhash3_x86_32("".as_bytes(), 0), 0);
    }

    #[test]
    fn test_tail_lengths() {
        assert_eq!(murmurhash3_x86_32("1".as_bytes(), 0), 2484513939);
        assert_eq!(murmurhash3_x86_32("12".as_bytes(), 0), 4191350549);
        assert_eq!(murmurhash3_x86_32("123".as_bytes(), 0), 2662625771);
        assert_eq!(murmurhash3_x86_32("1234".as_bytes(), 0), 1914461635);
    }

    #[test]
    fn test_large_data() {
        assert_eq!(murmurhash3_x86_32("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Etiam at consequat massa. Cras eleifend pellentesque ex, at dignissim libero maximus ut. Sed eget nulla felis".as_bytes(), 0), 1004899618);
    }
}
