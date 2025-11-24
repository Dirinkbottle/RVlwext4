// EXT4_CRC32 module

const crc32_table: [u32; 256] = [
    0x00000000, 0x77073096, 0xee0e612c, 0x990951ba, 0x076dc419, 0x706af48f, 0xe963a535, 0x9e6495a3,
    0x0edb8832, 0x79dcb8a4, 0xe0d5e91e, 0x97d2d988, 0x09b64c2b, 0x7eb17cbd, 0xe7b82d07, 0x90bf1d91,
    0x1db71064, 0x6ab020f2, 0xf3b97148, 0x84be41de, 0x1adad47d, 0x6ddde4eb, 0xf4d4b551, 0x83d385c7,
    0x136c9856, 0x646ba8c0, 0xfd62f97a, 0x8a65c9ec, 0x14015c4f, 0x63066cd9, 0xfa0f3d63, 0x8d080df5,
    0x3b6e20c8, 0x4c69105e, 0xd56041e4, 0xa2677172, 0x3c03e4d1, 0x4b04d447, 0xd20d85fd, 0xa50ab56b,
    0x35b5a8fa, 0x42b2986c, 0xdbbbc9d6, 0xacbcf940, 0x32d86ce3, 0x45df5c75, 0xdcd60dcf, 0xabd13d59,
    0x26d930ac, 0x51de003a, 0xc8d75180, 0xbfd06116, 0x21b4f4b5, 0x56b3c423, 0xcfba9599, 0xb8bda50f,
    0x2802b89e, 0x5f058808, 0xc60cd9b2, 0xb10be924, 0x2f6f7c87, 0x58684c11, 0xc1611dab, 0xb6662d3d,
    0x76dc4190, 0x01db7106, 0x98d220bc, 0xefd5102a, 0x71b18589, 0x06b6b51f, 0x9fbfe4a5, 0xe8b8d433,
    0x7807c9a2, 0x0f00f934, 0x9609a88e, 0xe10e9818, 0x7f6a0dbb, 0x086d3d2d, 0x91646c97, 0xe6635c01,
    0x6b6b51f4, 0x1c6c6162, 0x856530d8, 0xf262004e, 0x6c0695ed, 0x1b01a57b, 0x8208f4c1, 0xf50fc457,
    0x65b0d9c6, 0x12b7e950, 0x8bbeb8ea, 0xfcb9887c, 0x62dd1ddf, 0x15da2d49, 0x8cd37cf3, 0xfbd44c65,
    0x4db26158, 0x3ab551ce, 0xa3bc0074, 0xd4bb30e2, 0x4adfa541, 0x3dd895d7, 0xa4d1c46d, 0xd3d6f4fb,
    0x4369e96a, 0x346ed9fc, 0xad678846, 0xda60b8d0, 0x44042d73, 0x33031de5, 0xaa0a4c5f, 0xdd0d7cc9,
    0x5005713c, 0x270241aa, 0xbe0b1010, 0xc90c2086, 0x5768b525, 0x206f85b3, 0xb966d409, 0xce61e49f,
    0x5edef90e, 0x29d9c998, 0xb0d09822, 0xc7d7a8b4, 0x59b33d17, 0x2eb40d81, 0xb7bd5c3b, 0xc0ba6cad,
    0xedb88320, 0x9abfb3b6, 0x03b6e20c, 0x74b1d29a, 0xead54739, 0x9dd277af, 0x04db2615, 0x73dc1683,
    0xe3630b12, 0x94643b84, 0x0d6d6a3e, 0x7a6a5aa8, 0xe40ecf0b, 0x9309ff9d, 0x0a00ae27, 0x7d079eb1,
    0xf00f9344, 0x8708a3d2, 0x1e01f268, 0x6906c2fe, 0xf762575d, 0x806567cb, 0x196c3671, 0x6e6b06e7,
    0xfed41b76, 0x89d32be0, 0x10da7a5a, 0x67dd4acc, 0xf9b9df6f, 0x8ebeeff9, 0x17b7be43, 0x60b08ed5,
    0xd6d6a3e8, 0xa1d1937e, 0x38d8c2c4, 0x4fdff252, 0xd1bb67f1, 0xa6bc5767, 0x3fb506dd, 0x48b2364b,
    0xd80d2bda, 0xaf0a1b4c, 0x36034af6, 0x41047a60, 0xdf60efc3, 0xa867df55, 0x316e8eef, 0x4669be79,
    0xcb61b38c, 0xbc66831a, 0x256fd2a0, 0x5268e236, 0xcc0c7795, 0xbb0b4703, 0x220216b9, 0x5505262f,
    0xc5ba3bbe, 0xb2bd0b28, 0x2bb45a92, 0x5cb36a04, 0xc2d7ffa7, 0xb5d0cf31, 0x2cd99e8b, 0x5bdeae1d,
    0x9b64c2b0, 0xec63f226, 0x756aa39c, 0x026d930a, 0x9c0906a9, 0xeb0e363f, 0x72076785, 0x05005713,
    0x95bf4a82, 0xe2b87a14, 0x7bb12bae, 0x0cb61b38, 0x92d28e9b, 0xe5d5be0d, 0x7cdcefb7, 0x0bdbdf21,
    0x86d3d2d4, 0xf1d4e242, 0x68ddb3f8, 0x1fda836e, 0x81be16cd, 0xf6b9265b, 0x6fb077e1, 0x18b74777,
    0x88085ae6, 0xff0f6a70, 0x66063bca, 0x11010b5c, 0x8f659eff, 0xf862ae69, 0x616bffd3, 0x166ccf45,
    0xa00ae278, 0xd70dd2ee, 0x4e048354, 0x3903b3c2, 0xa7672661, 0xd06016f7, 0x4969474d, 0x3e6e77db,
    0xaed16a4a, 0xd9d65adc, 0x40df0b66, 0x37d83bf0, 0xa9bcae53, 0xdebb9ec5, 0x47b2cf7f, 0x30b5ffe9,
    0xbdbdf21c, 0xcabac28a, 0x53b39330, 0x24b4a3a6, 0xbad03605, 0xcdd70693, 0x54de5729, 0x23d967bf,
    0xb3667a2e, 0xc4614ab8, 0x5d681b02, 0x2a6f2b94, 0xb40bbe37, 0xc30c8ea1, 0x5a05df1b, 0x2d02ef8d,
];

const crc32c_table: [u32; 256] = [
    0x00000000, 0xF26B8303, 0xE13B70F7, 0x1350F3F4, 0xC79A971F, 0x35F1141C, 0x26A1E7E8, 0xD4CA64EB,
    0x8AD958CF, 0x78B2DBCC, 0x6BE22838, 0x9989AB3B, 0x4D43CFD0, 0xBF284CD3, 0xAC78BF27, 0x5E133C24,
    0x105EC76F, 0xE235446C, 0xF165B798, 0x030E349B, 0xD7C45070, 0x25AFD373, 0x36FF2087, 0xC494A384,
    0x9A879FA0, 0x68EC1CA3, 0x7BBCEF57, 0x89D76C54, 0x5D1D08BF, 0xAF768BBC, 0xBC267848, 0x4E4DFB4B,
    0x20BD8EDE, 0xD2D60DDD, 0xC186FE29, 0x33ED7D2A, 0xE72719C1, 0x154C9AC2, 0x061C6936, 0xF477EA35,
    0xAA64D611, 0x580F5512, 0x4B5FA6E6, 0xB93425E5, 0x6DFE410E, 0x9F95C20D, 0x8CC531F9, 0x7EAEB2FA,
    0x30E349B1, 0xC288CAB2, 0xD1D83946, 0x23B3BA45, 0xF779DEAE, 0x05125DAD, 0x1642AE59, 0xE4292D5A,
    0xBA3A117E, 0x4851927D, 0x5B016189, 0xA96AE28A, 0x7DA08661, 0x8FCB0562, 0x9C9BF696, 0x6EF07595,
    0x417B1DBC, 0xB3109EBF, 0xA0406D4B, 0x522BEE48, 0x86E18AA3, 0x748A09A0, 0x67DAFA54, 0x95B17957,
    0xCBA24573, 0x39C9C670, 0x2A993584, 0xD8F2B687, 0x0C38D26C, 0xFE53516F, 0xED03A29B, 0x1F682198,
    0x5125DAD3, 0xA34E59D0, 0xB01EAA24, 0x42752927, 0x96BF4DCC, 0x64D4CECF, 0x77843D3B, 0x85EFBE38,
    0xDBFC821C, 0x2997011F, 0x3AC7F2EB, 0xC8AC71E8, 0x1C661503, 0xEE0D9600, 0xFD5D65F4, 0x0F36E6F7,
    0x61C69362, 0x93AD1061, 0x80FDE395, 0x72966096, 0xA65C047D, 0x5437877E, 0x4767748A, 0xB50CF789,
    0xEB1FCBAD, 0x197448AE, 0x0A24BB5A, 0xF84F3859, 0x2C855CB2, 0xDEEEDFB1, 0xCDBE2C45, 0x3FD5AF46,
    0x7198540D, 0x83F3D70E, 0x90A324FA, 0x62C8A7F9, 0xB602C312, 0x44694011, 0x5739B3E5, 0xA55230E6,
    0xFB410CC2, 0x092A8FC1, 0x1A7A7C35, 0xE811FF36, 0x3CDB9BDD, 0xCEB018DE, 0xDDE0EB2A, 0x2F8B6829,
    0x82F63B78, 0x709DB87B, 0x63CD4B8F, 0x91A6C88C, 0x456CAC67, 0xB7072F64, 0xA457DC90, 0x563C5F93,
    0x082F63B7, 0xFA44E0B4, 0xE9141340, 0x1B7F9043, 0xCFB5F4A8, 0x3DDE77AB, 0x2E8E845F, 0xDCE5075C,
    0x92A8FC17, 0x60C37F14, 0x73938CE0, 0x81F80FE3, 0x55326B08, 0xA759E80B, 0xB4091BFF, 0x466298FC,
    0x1871A4D8, 0xEA1A27DB, 0xF94AD42F, 0x0B21572C, 0xDFEB33C7, 0x2D80B0C4, 0x3ED04330, 0xCCBBC033,
    0xA24BB5A6, 0x502036A5, 0x4370C551, 0xB11B4652, 0x65D122B9, 0x97BAA1BA, 0x84EA524E, 0x7681D14D,
    0x2892ED69, 0xDAF96E6A, 0xC9A99D9E, 0x3BC21E9D, 0xEF087A76, 0x1D63F975, 0x0E330A81, 0xFC588982,
    0xB21572C9, 0x407EF1CA, 0x532E023E, 0xA145813D, 0x758FE5D6, 0x87E466D5, 0x94B49521, 0x66DF1622,
    0x38CC2A06, 0xCAA7A905, 0xD9F75AF1, 0x2B9CD9F2, 0xFF56BD19, 0x0D3D3E1A, 0x1E6DCDEE, 0xEC064EED,
    0xC38D26C4, 0x31E6A5C7, 0x22B65633, 0xD0DDD530, 0x0417B1DB, 0xF67C32D8, 0xE52CC12C, 0x1747422F,
    0x49547E0B, 0xBB3FFD08, 0xA86F0EFC, 0x5A048DFF, 0x8ECEE914, 0x7CA56A17, 0x6FF599E3, 0x9D9E1AE0,
    0xD3D3E1AB, 0x21B862A8, 0x32E8915C, 0xC083125F, 0x144976B4, 0xE622F5B7, 0xF5720643, 0x07198540,
    0x590AB964, 0xAB613A67, 0xB831C993, 0x4A5A4A90, 0x9E902E7B, 0x6CFBAD78, 0x7FAB5E8C, 0x8DC0DD8F,
    0xE330A81A, 0x115B2B19, 0x020BD8ED, 0xF0605BEE, 0x24AA3F05, 0xD6C1BC06, 0xC5914FF2, 0x37FACCF1,
    0x69E9F0D5, 0x9B8273D6, 0x88D28022, 0x7AB90321, 0xAE7367CA, 0x5C18E4C9, 0x4F48173D, 0xBD23943E,
    0xF36E6F75, 0x0105EC76, 0x12551F82, 0xE03E9C81, 0x34F4F86A, 0xC69F7B69, 0xD5CF889D, 0x27A40B9E,
    0x79B737BA, 0x8BDCB4B9, 0x988C474D, 0x6AE7C44E, 0xBE2DA0A5, 0x4C4623A6, 0x5F16D052, 0xAD7D5351,
];

fn crc32(crc: u32, buf: &[u8], table: &[u32; 256]) -> u32 {
    let mut c = crc;

    for &byte in buf.iter() {
        let index = ((c ^ (byte as u32)) & 0xff) as usize;
        c = (c >> 8) ^ table[index];
    }
    c
}

pub fn ext4_crc32(crc: u32, buf: &[u8]) -> u32 {
    crc32(crc, buf, &crc32_table)
}

pub fn ext4_crc32c(crc: u32, buf: &[u8]) -> u32 {
    crc32(crc, buf, &crc32c_table)
}

/// 通用的结构体到字节切片转换工具
/// 计算从结构体开始到指定字段前的字节切片
pub fn struct_bytes_before_filed<T>(data: &T, filed_offset: usize) -> &[u8] {
    let ptr = data as *const _ as *const u8;
    unsafe { core::slice::from_raw_parts(ptr, filed_offset) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::offsetof;

    // ============ 基础功能测试 ============
    #[test]
    fn test_struct_bytes_before_filed() {
        // 测试 1：默认 Rust 布局（有填充）
        {
            #[repr(C)] //防止编译器重排字段！
            pub struct TestStruct {
                field_1: u32,
                field_2: u8,
                field_3: u16,
                field_4: u8,
            }

            let ts = TestStruct {
                field_1: 0x11223344,
                field_2: 0x55,
                field_3: 0x6677,
                field_4: 0x88,
            };

            // 验证偏移量
            assert_eq!(offsetof!(TestStruct, field_1), 0);
            assert_eq!(offsetof!(TestStruct, field_2), 4);
            assert_eq!(offsetof!(TestStruct, field_3), 6); // 有 1 字节填充
            assert_eq!(offsetof!(TestStruct, field_4), 8);

            // 测试字节提取
            let bytes = struct_bytes_before_filed(&ts, offsetof!(TestStruct, field_3));
            assert_eq!(bytes.len(), 6);
            assert_eq!(bytes, [0x44, 0x33, 0x22, 0x11, 0x55, 0x00]); // 小端序 + 填充
        }

        // 测试 2：紧凑布局（无填充）
        {
            #[repr(C, packed)]
            pub struct PackedStruct {
                field_1: u32,
                field_2: u8,
                field_3: u16,
                field_4: u8,
            }

            let ps = PackedStruct {
                field_1: 0x11223344,
                field_2: 0x55,
                field_3: 0x6677,
                field_4: 0x88,
            };

            // 验证偏移量
            assert_eq!(offsetof!(PackedStruct, field_1), 0);
            assert_eq!(offsetof!(PackedStruct, field_2), 4);
            assert_eq!(offsetof!(PackedStruct, field_3), 5); // 无填充
            assert_eq!(offsetof!(PackedStruct, field_4), 7);

            // 测试字节提取
            let bytes = struct_bytes_before_filed(&ps, offsetof!(PackedStruct, field_3));
            assert_eq!(bytes.len(), 5);
            assert_eq!(bytes, [0x44, 0x33, 0x22, 0x11, 0x55]); // 小端序，无填充
        }
    }

    #[test]
    fn test_crc32_empty_with_zero_init() {
        // 空数据 + 零初始值应该返回零
        let result = ext4_crc32(0, &[]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_crc32_empty_preserves_init() {
        // 空数据应该保持初始值不变
        let init_values = [0x00000000, 0xFFFFFFFF, 0x12345678, 0xABCDEF00];

        for init in init_values {
            let result = ext4_crc32(init, &[]);
            assert_eq!(
                result, init,
                "Empty data should preserve init value 0x{:08X}",
                init
            );
        }
    }

    #[test]
    fn test_crc32c_empty_preserves_init() {
        // 空数据应该保持初始值不变
        let init_values = [0x00000000, 0xFFFFFFFF, 0x12345678, 0xABCDEF00];

        for init in init_values {
            let result = ext4_crc32c(init, &[]);
            assert_eq!(
                result, init,
                "Empty data should preserve init value 0x{:08X}",
                init
            );
        }
    }

    // ============ 一致性测试 ============

    #[test]
    fn test_crc32_consistency() {
        // 相同输入应该产生相同输出
        let test_data = [
            b"test" as &[u8],
            b"Hello, World!",
            b"123456789",
            b"",
            &[0x00, 0xFF, 0xAA, 0x55],
        ];

        for data in test_data {
            let result1 = ext4_crc32(0, data);
            let result2 = ext4_crc32(0, data);
            let result3 = ext4_crc32(0, data);

            assert_eq!(
                result1, result2,
                "CRC32 should be consistent for: {:?}",
                data
            );
            assert_eq!(
                result2, result3,
                "CRC32 should be consistent for: {:?}",
                data
            );
        }
    }

    #[test]
    fn test_crc32c_consistency() {
        // 相同输入应该产生相同输出
        let test_data = [
            b"test" as &[u8],
            b"Hello, World!",
            b"123456789",
            b"",
            &[0x00, 0xFF, 0xAA, 0x55],
        ];

        for data in test_data {
            let result1 = ext4_crc32c(0, data);
            let result2 = ext4_crc32c(0, data);
            let result3 = ext4_crc32c(0, data);

            assert_eq!(
                result1, result2,
                "CRC32C should be consistent for: {:?}",
                data
            );
            assert_eq!(
                result2, result3,
                "CRC32C should be consistent for: {:?}",
                data
            );
        }
    }

    // ============ 增量计算测试 ============

    #[test]
    fn test_crc32_incremental_2_parts() {
        // 分两段计算应该和一次性计算相同
        let data = b"Hello, World! This is a test message.";

        let full = ext4_crc32(0, data);

        let part1 = ext4_crc32(0, &data[0..10]);
        let part2 = ext4_crc32(part1, &data[10..]);

        assert_eq!(full, part2, "2-part incremental CRC32 should match full");
    }

    #[test]
    fn test_crc32_incremental_3_parts() {
        // 分三段计算
        let data = b"The quick brown fox jumps over the lazy dog";

        let full = ext4_crc32(0, data);

        let crc1 = ext4_crc32(0, &data[0..15]);
        let crc2 = ext4_crc32(crc1, &data[15..30]);
        let crc3 = ext4_crc32(crc2, &data[30..]);

        assert_eq!(full, crc3, "3-part incremental CRC32 should match full");
    }

    #[test]
    fn test_crc32c_incremental() {
        // CRC32C 增量计算
        let data = b"abcdefghijklmnopqrstuvwxyz0123456789";

        let full = ext4_crc32c(0, data);

        let crc1 = ext4_crc32c(0, &data[0..10]);
        let crc2 = ext4_crc32c(crc1, &data[10..20]);
        let crc3 = ext4_crc32c(crc2, &data[20..]);

        assert_eq!(full, crc3, "Incremental CRC32C should match full");
    }

    #[test]
    fn test_crc32_incremental_byte_by_byte() {
        // 逐字节计算
        let data = b"test123";

        let full = ext4_crc32(0, data);

        let mut crc = 0;
        for &byte in data.iter() {
            crc = ext4_crc32(crc, &[byte]);
        }

        assert_eq!(full, crc, "Byte-by-byte CRC32 should match full");
    }

    // ============ 不同初始值测试 ============

    #[test]
    fn test_crc32_different_init_values() {
        // 不同初始值应该产生不同结果
        let data = b"test data";

        let crc_init0 = ext4_crc32(0x00000000, data);
        let crc_initF = ext4_crc32(0xFFFFFFFF, data);
        let crc_init1 = ext4_crc32(0x12345678, data);
        let crc_init2 = ext4_crc32(0xABCDEF00, data);

        assert_ne!(crc_init0, crc_initF);
        assert_ne!(crc_init0, crc_init1);
        assert_ne!(crc_init0, crc_init2);
        assert_ne!(crc_initF, crc_init1);
    }

    #[test]
    fn test_crc32c_different_init_values() {
        // 不同初始值应该产生不同结果
        let data = b"test data";

        let crc_init0 = ext4_crc32c(0x00000000, data);
        let crc_initF = ext4_crc32c(0xFFFFFFFF, data);
        let crc_init1 = ext4_crc32c(0x12345678, data);

        assert_ne!(crc_init0, crc_initF);
        assert_ne!(crc_init0, crc_init1);
        assert_ne!(crc_initF, crc_init1);
    }

    // ============ 数据敏感性测试 ============

    #[test]
    fn test_crc32_data_sensitivity() {
        // 数据变化应该导致 CRC 变化
        let data1 = b"test";
        let data2 = b"TEST";
        let data3 = b"test ";
        let data4 = b"tes";

        let crc1 = ext4_crc32(0, data1);
        let crc2 = ext4_crc32(0, data2);
        let crc3 = ext4_crc32(0, data3);
        let crc4 = ext4_crc32(0, data4);

        assert_ne!(crc1, crc2, "Case change should affect CRC");
        assert_ne!(crc1, crc3, "Extra space should affect CRC");
        assert_ne!(crc1, crc4, "Shorter data should affect CRC");
    }

    #[test]
    fn test_crc32_single_bit_flip() {
        // 单比特翻转应该改变 CRC
        let mut data1 = [0b10101010u8; 16];
        let crc1 = ext4_crc32(0, &data1);

        // 翻转一个比特
        data1[8] ^= 0b00000001;
        let crc2 = ext4_crc32(0, &data1);

        assert_ne!(crc1, crc2, "Single bit flip should change CRC");
    }

    #[test]
    fn test_crc32_order_matters() {
        // 字节顺序改变应该影响 CRC
        let data1 = b"abc";
        let data2 = b"bac";
        let data3 = b"cba";

        let crc1 = ext4_crc32(0, data1);
        let crc2 = ext4_crc32(0, data2);
        let crc3 = ext4_crc32(0, data3);

        assert_ne!(crc1, crc2);
        assert_ne!(crc1, crc3);
        assert_ne!(crc2, crc3);
    }

    // ============ CRC32 vs CRC32C 对比测试 ============

    #[test]
    fn test_crc32_vs_crc32c_different() {
        // CRC32 和 CRC32C 对相同数据应该产生不同结果
        let test_data = [
            b"" as &[u8],
            b"a",
            b"test",
            b"Hello, World!",
            b"123456789",
            &[0x00, 0xFF, 0xAA, 0x55],
        ];

        for data in test_data {
            let crc32 = ext4_crc32(0, data);
            let crc32c = ext4_crc32c(0, data);

            // 除了空数据，其他都应该不同
            if !data.is_empty() {
                assert_ne!(
                    crc32, crc32c,
                    "CRC32 and CRC32C should differ for non-empty data: {:?}",
                    data
                );
            }
        }
    }

    // ============ 特殊数据模式测试 ============

    #[test]
    fn test_crc32_all_zeros() {
        let data = [0u8; 64];

        // 零初始值 + 全零数据 = 零 CRC（这是正常的）
        let crc_zero_init = ext4_crc32(0, &data);
        assert_eq!(
            crc_zero_init, 0,
            "All-zero data with zero init produces zero CRC"
        );

        // 但是如果用非零初始值，应该产生非零结果
        let crc_ff_init = ext4_crc32(0xFFFFFFFF, &data);
        assert_ne!(
            crc_ff_init, 0,
            "All-zero data with 0xFFFFFFFF init should produce non-zero CRC"
        );
        assert_ne!(
            crc_ff_init, 0xFFFFFFFF,
            "Result should differ from init value"
        );

        // 一致性检查
        let crc2 = ext4_crc32(0xFFFFFFFF, &data);
        assert_eq!(crc_ff_init, crc2, "CRC should be consistent");
    }

    #[test]
    fn test_crc32c_all_zeros() {
        let data = [0u8; 64];

        // 零初始值 + 全零数据 = 零 CRC
        let crc_zero_init = ext4_crc32c(0, &data);
        assert_eq!(crc_zero_init, 0);

        // 非零初始值应该产生非零结果
        let crc_ff_init = ext4_crc32c(0xFFFFFFFF, &data);
        assert_ne!(crc_ff_init, 0);

        // 一致性检查
        let crc2 = ext4_crc32c(0xFFFFFFFF, &data);
        assert_eq!(crc_ff_init, crc2);
    }

    #[test]
    fn test_crc32_all_ones() {
        let data = [0xFFu8; 64];
        let crc = ext4_crc32(0, &data);

        // 一致性检查
        let crc2 = ext4_crc32(0, &data);
        assert_eq!(crc, crc2);
    }

    #[test]
    fn test_crc32_alternating_pattern() {
        let mut data = [0u8; 64];
        for i in 0..64 {
            data[i] = if i % 2 == 0 { 0xAA } else { 0x55 };
        }

        let crc1 = ext4_crc32(0, &data);
        let crc2 = ext4_crc32(0, &data);

        assert_eq!(crc1, crc2);
        assert_ne!(crc1, 0);
    }

    #[test]
    fn test_crc32_sequential_bytes() {
        let mut data = [0u8; 256];
        for i in 0..256 {
            data[i] = i as u8;
        }

        let crc1 = ext4_crc32(0, &data);
        let crc2 = ext4_crc32(0, &data);

        assert_eq!(crc1, crc2);
    }

    // ============ 长度敏感性测试 ============

    #[test]
    fn test_crc32_length_sensitivity() {
        let base = b"0123456789ABCDEF";

        let crc1 = ext4_crc32(0, &base[0..1]);
        let crc2 = ext4_crc32(0, &base[0..4]);
        let crc3 = ext4_crc32(0, &base[0..8]);
        let crc4 = ext4_crc32(0, &base[0..16]);

        // 不同长度应该产生不同 CRC
        assert_ne!(crc1, crc2);
        assert_ne!(crc2, crc3);
        assert_ne!(crc3, crc4);
    }

    #[test]
    fn test_crc32_single_byte_values() {
        // 不同的单字节应该产生不同的 CRC
        let bytes = [0x00, 0x01, 0x7F, 0x80, 0xFF];
        let mut crcs = [0u32; 5];

        for (i, &byte) in bytes.iter().enumerate() {
            crcs[i] = ext4_crc32(0, &[byte]);
        }

        // 检查所有 CRC 都不相同
        for i in 0..5 {
            for j in (i + 1)..5 {
                assert_ne!(
                    crcs[i], crcs[j],
                    "CRC of 0x{:02X} should differ from CRC of 0x{:02X}",
                    bytes[i], bytes[j]
                );
            }
        }
    }

    // ============ 链式计算测试 ============

    #[test]
    fn test_crc32_chain_calculation() {
        // 模拟处理多个块
        let block1 = [0x11u8; 512];
        let block2 = [0x22u8; 512];
        let block3 = [0x33u8; 512];

        // 链式计算
        let crc = ext4_crc32(0, &block1);
        let crc = ext4_crc32(crc, &block2);
        let crc = ext4_crc32(crc, &block3);

        // 一次性计算
        let mut full_data = [0u8; 1536];
        full_data[0..512].copy_from_slice(&block1);
        full_data[512..1024].copy_from_slice(&block2);
        full_data[1024..1536].copy_from_slice(&block3);
        let crc_full = ext4_crc32(0, &full_data);

        assert_eq!(crc, crc_full, "Chain calculation should match full");
    }

    // ============ EXT4 实际使用场景测试 ============

    #[test]
    fn test_crc32_ext4_superblock_simulation() {
        // 模拟 EXT4 超级块校验（1024 字节）
        let superblock = [0u8; 1024];

        // 使用 0xFFFFFFFF 初始值（EXT4 常见做法）
        let crc1 = ext4_crc32(0xFFFFFFFF, &superblock);
        let crc2 = ext4_crc32(0xFFFFFFFF, &superblock);

        assert_eq!(crc1, crc2, "Superblock CRC should be reproducible");
    }

    #[test]
    fn test_crc32c_journal_block_simulation() {
        // 模拟日志块校验（4096 字节）
        let journal_block = [0xAAu8; 4096];

        let crc = ext4_crc32c(0, &journal_block);

        // 修改一个字节
        let mut modified_block = [0xAAu8; 4096];
        modified_block[2048] = 0xBB;
        let crc_modified = ext4_crc32c(0, &modified_block);

        assert_ne!(
            crc, crc_modified,
            "Modified block should have different CRC"
        );
    }

    #[test]
    fn test_crc32_block_device_sector() {
        // 模拟块设备扇区（512 字节）
        let sector = [0x00u8; 512];
        let crc_orig = ext4_crc32(0, &sector);

        // 修改扇区
        let mut sector2 = [0x00u8; 512];
        sector2[256] = 0x01;
        let crc_modified = ext4_crc32(0, &sector2);

        assert_ne!(
            crc_orig, crc_modified,
            "Sector modification should change CRC"
        );
    }

    // ============ 大数据测试 ============

    #[test]
    fn test_crc32_large_data() {
        // 测试 4KB 数据
        let data = [0x5Au8; 4096];

        let crc1 = ext4_crc32(0, &data);
        let crc2 = ext4_crc32(0, &data);

        assert_eq!(crc1, crc2, "Large data CRC should be consistent");
        assert_ne!(crc1, 0, "Large data should produce non-zero CRC");
    }

    #[test]
    fn test_crc32c_large_data() {
        // 测试 8KB 数据
        let data = [0xA5u8; 8192];

        let crc1 = ext4_crc32c(0, &data);
        let crc2 = ext4_crc32c(0, &data);

        assert_eq!(crc1, crc2, "Large data CRC32C should be consistent");
    }

    // ============ 边界条件测试 ============

    #[test]
    fn test_crc32_boundary_bytes() {
        // 测试边界字节值
        let test_bytes = [0x00, 0x01, 0x7F, 0x80, 0xFE, 0xFF];

        for &byte in &test_bytes {
            let crc = ext4_crc32(0, &[byte]);

            // 所有单字节都应该产生非零 CRC（除非算法本身有特殊性）
            // 这里主要测试不会 panic
            let _ = crc;
        }
    }

    #[test]
    fn test_crc32_repeating_pattern() {
        // 测试重复模式
        let mut data = [0u8; 120];
        for i in 0..40 {
            data[i * 3] = b'A';
            data[i * 3 + 1] = b'B';
            data[i * 3 + 2] = b'C';
        }

        let crc1 = ext4_crc32(0, &data);
        let crc2 = ext4_crc32(0, &data);

        assert_eq!(crc1, crc2);
    }
}
