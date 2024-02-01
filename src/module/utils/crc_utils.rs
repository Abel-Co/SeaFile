use crc::*;
use once_cell::sync::OnceCell;

/// 数据库只能bigint存储，无论性能还是兼容性。
/// So 这里也只暴露 i64，转换时 “溢出位” 将作为负数。
pub fn crc_i64(str: &str) -> i64 {
    crc().checksum(str.as_bytes()) as i64
}

fn crc<'a>() -> &'a Crc<u64> {
    static CRC: OnceCell<Crc<u64>> = OnceCell::new();
    CRC.get_or_init(|| {
        Crc::<u64>::new(&CRC_64_XZ)
    })
}


// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod test_crc {
    use crc::*;

    const INIT: &[u8] = b"123456789";

// #[test]
// fn crc_8() {
//     let algs = &[CRC_8_AUTOSAR, CRC_8_BLUETOOTH, CRC_8_SMBUS, CRC_8_DARC];
//     for alg in algs {
//         let crc = Crc::<u8>::new(alg);
//         assert_eq!(alg.check, crc.checksum(INIT));
//         let mut digest = crc.digest();
//         digest.update(INIT);
//         assert_eq!(alg.check, digest.finalize());
//     }
// }

    #[test]
    fn crc_16() {
        let algs = &[
            CRC_16_IBM_SDLC,
            CRC_16_USB,
            CRC_16_ARC,
            CRC_16_CDMA2000,
            CRC_16_IBM_3740,
            CRC_16_KERMIT,
        ];
        for alg in algs {
            let crc = Crc::<u16>::new(alg);
            assert_eq!(alg.check, crc.checksum(INIT));
            let mut digest = crc.digest();
            digest.update(INIT);
            // assert_eq!(alg.check, digest.finalize());
            println!("{:?}, {:?}, {:?}", alg.check, crc.checksum(INIT), digest.finalize())
        }
    }

    #[test]
    fn crc_32() {
        let algs = &[CRC_32_ISCSI, CRC_32_AUTOSAR, CRC_32_BZIP2, CRC_32_ISO_HDLC, ];
        for alg in algs {
            let crc = Crc::<u32>::new(alg);
            assert_eq!(alg.check, crc.checksum(INIT));
            let mut digest = crc.digest();
            digest.update(INIT);
            // assert_eq!(alg.check, digest.finalize());
            println!("{:?}, {:?}, {:?}", alg.check, crc.checksum(INIT), digest.finalize())
        }
    }

    #[test]
    fn crc_64() {
        let algs = &[CRC_64_ECMA_182, CRC_64_GO_ISO, CRC_64_WE, CRC_64_XZ];
        for alg in algs {
            let crc = Crc::<u64>::new(alg);
            assert_eq!(alg.check, crc.checksum(INIT));
            let mut digest = crc.digest();
            digest.update(INIT);
            // assert_eq!(alg.check, digest.finalize());
            // println!("{:?}, {:?}, {:?}", alg.check, crc.checksum(INIT), digest.finalize());
            let str = "中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国";
            println!("{:?}", crc.checksum(str.as_bytes()));
            digest.update(str.as_ref());
            println!("{:?}", digest.finalize());

            // println!("{:?}", encryption::md5(str));
        }
    }

    #[test]
    fn crc1() {
        let crc = Crc::<u64>::new(&CRC_64_GO_ISO);
        let mut digest = crc.digest();

        digest.update(INIT);

        let str = "中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国";
        digest.update(str.as_bytes());
        digest.update(str.as_bytes());  // 每次不一样
        digest.update(str.as_bytes());  // 每次不一样，不能用作校验
        println!("{:?}", digest.finalize());
    }

    #[test]
    fn crc2() {
        let crc = Crc::<u64>::new(&CRC_64_XZ);  // CRC_64_XZ、CRC_64_ECMA_182：都可在crc.checksum(str.as_bytes()中，每次获得19位整数
        let str = "中华人民共和国中华人民共和国中华人和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国中华人民共和国";
        println!("{:?}", crc.checksum(str.as_bytes()));
        println!("{:?}", crc.checksum(str.as_bytes()));
        println!("{:?}", crc.checksum(str.as_bytes()));
        // let c: u64 = 12317007190264842376;
    }

// 下面方法，都可获得19位整数。
// 但：digest.finalize() 每次不一样，so 不能用作校验
//
// # CRC_64_ECMA_182, crc.checksum(str.as_bytes())
// 9577108474319817938
//
// # CRC_64_GO_ISO, digest.finalize()
// 2419424015746097939
//
// # CRC_64_WE, digest.finalize()
// 2758515223301323332
//
// # CRC_64_XZ, crc.checksum(str.as_bytes())
// 4012298080266222156
}