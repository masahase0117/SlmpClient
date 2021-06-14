use crate::SLMPDeviceCode;
use std::cmp::Ordering;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unpack_words_by_word() {
        let data = [0x34u8, 0x12, 0x02, 0x00, 0xef, 0x1d];
        let ret = [0x1234u16, 0x0002, 0x1def];
        let buf = unpack_words_by_word(&data);
        assert_eq!(buf.as_slice(), ret);
    }
    #[test]
    fn test_unpack_bits_by_word() {
        let data = [0x34u8, 0x12];
        let ret = [
            false, false, true, false, true, true, false, false, false, true, false, false, true,
            false, false, false,
        ];
        let buf = unpack_bits_by_word(&data);
        assert_eq!(buf.as_slice(), ret)
    }
    #[test]
    fn test_unpack_bits_by_bit() {
        let data = [0x00u8, 0x01, 0x00, 0x11];
        let ret = [false, false, false, true, false, false, true, true];
        let buf = unpack_bits_by_bit(&data).unwrap();
        assert_eq!(buf.as_slice(), ret)
    }
    #[test]
    fn test_pack_bits_by_bit() {
        let data = [false, false, false, true, false, false, true, true];
        let ret = [0x00u8, 0x01, 0x00, 0x11];
        let buf = pack_bits_by_bit(&data);
        assert_eq!(buf.as_slice(), ret)
    }
    #[test]
    fn test_bits_by_bit() {
        let data = [false, false, false, true, false, false, true, true];
        let buf = pack_bits_by_bit(&data);
        let ret = unpack_bits_by_bit(buf.as_slice()).unwrap();
        assert_eq!(ret.as_slice(), data)
    }
    #[test]
    fn test_pack_bits_by_word() {
        let data = [true, true, false, true, false, false];
        let ret = [0b00001011u8];
        let buf = pack_bits_by_word(&data);
        assert_eq!(buf.as_slice(), ret)
    }
    #[test]
    fn test_bits_by_word() {
        let data = [true, true, false, true, false, false, true, false];
        let buf = pack_bits_by_word(&data);
        let ret = unpack_bits_by_word(buf.as_slice());
        assert_eq!(ret.as_slice(), data)
    }
    #[test]
    fn test_pack_words_by_word() {
        let data = [0x1995u16, 0x1202u16, 0x1130u16];
        let ret = [0x95u8, 0x19, 0x02, 0x12, 0x30, 0x11];
        let buf = pack_words_by_word(&data);
        assert_eq!(buf.as_slice(), ret)
    }
    #[test]
    fn test_words_by_word() {
        let data = [0x1995u16, 0x1202u16, 0x1130u16];
        let buf = pack_words_by_word(&data);
        let ret = unpack_words_by_word(buf.as_slice());
        assert_eq!(ret.as_slice(), data)
    }
    #[test]
    fn test_unpack_dwords_by_dword() {
        let data = [0x4e, 0x4f, 0x54, 0x4c, 0xaf, 0xb9, 0xde, 0xc3];
        let ret = [0x4c544f4e, 0xc3deb9af];
        let buf = unpack_dwords_by_dword(&data);
        assert_eq!(buf.as_slice(), ret);
    }
    #[test]
    fn test_unpack_bits_in_byte() {
        let data = 0x49;
        let ret = [true, false, false, true, false, false, true, false];
        let buf = unpack_bits_in_byte(data);
        assert_eq!(buf, ret);
        let data = 0xaf;
        let ret = [true, true, true, true, false, true, false, true];
        let buf = unpack_bits_in_byte(data);
        assert_eq!(buf, ret);
    }
    #[test]
    fn test_unpack_bits_in_word() {
        let data = 0x2030;
        let ret = [
            false, false, false, false, true, true, false, false, false, false, false, false,
            false, true, false, false,
        ];
        let buf = unpack_bits_in_word(data);
        assert_eq!(buf, ret);
    }
    #[test]
    fn test_unpack_bits_in_dword() {
        let data = 0xc3deb9af;
        let ret = [
            true, true, true, true, false, true, false, true, true, false, false, true, true, true,
            false, true, false, true, true, true, true, false, true, true, true, true, false,
            false, false, false, true, true,
        ];
        let buf = unpack_bits_in_dword(data);
        assert_eq!(buf, ret)
    }
    #[test]
    fn test_unpack_word2bit() {
        let src = SLMPDeviceData::<u16> {
            dev: SLMPDevice {
                d_code: SLMPDeviceCode::X,
                addr: 0x20,
            },
            value: 0x4849,
        };
        let ret = unpack_word2bit(&src);
        let buf = [
            true, false, false, true, false, false, true, false, false, false, false, true, false,
            false, true, false,
        ];
        for i in 0..16 {
            assert_eq!(ret[i].dev.addr, src.dev.addr + i as u32);
            assert_eq!(ret[i].dev.d_code, src.dev.d_code);
            assert_eq!(ret[i].value, buf[i]);
        }
    }
    #[test]
    fn test_unpack_dword2bit() {
        let src = SLMPDeviceData::<u32> {
            dev: SLMPDevice {
                d_code: SLMPDeviceCode::M,
                addr: 1111,
            },
            value: 0xbaddbcb7,
        };
        let ret = unpack_dword2bit(&src);
        let buf = [
            true, true, true, false, true, true, false, true, false, false, true, true, true, true,
            false, true, true, false, true, true, true, false, true, true, false, true, false,
            true, true, true, false, true,
        ];
        for i in 0..32 {
            assert_eq!(ret[i].dev.addr, src.dev.addr + i as u32);
            assert_eq!(ret[i].dev.d_code, src.dev.d_code);
            assert_eq!(ret[i].value, buf[i]);
        }
    }
    #[test]
    fn test_unpack_dword2word() {
        let src = SLMPDeviceData::<u32> {
            dev: SLMPDevice {
                d_code: SLMPDeviceCode::D,
                addr: 1500,
            },
            value: 0x4c544f4e,
        };
        let ret = unpack_dword2word(&src);
        let buf = [
            SLMPDeviceData::<u16> {
                dev: SLMPDevice {
                    d_code: SLMPDeviceCode::D,
                    addr: 1500,
                },
                value: 0x4c54,
            },
            SLMPDeviceData::<u16> {
                dev: SLMPDevice {
                    d_code: SLMPDeviceCode::D,
                    addr: 1501,
                },
                value: 0x4f4e,
            },
        ];
        for i in 0..2 {
            assert_eq!(ret[i], buf[i])
        }
    }
    #[test]
    fn test_pack_bit2word() {
        let mut src = [SLMPDeviceData::<bool> {
            dev: SLMPDevice {
                d_code: SLMPDeviceCode::M,
                addr: 100,
            },
            value: true,
        }; 16];
        let buf = [
            false, false, false, false, true, true, false, false, false, false, false, false,
            false, true, false, false,
        ];
        for i in 0..16 {
            src[i].value = buf[i];
            src[i].dev.addr = 100 + i as u32;
        }
        let ret = pack_bit2word(&src);
        assert_eq!(
            ret.dev,
            SLMPDevice {
                d_code: SLMPDeviceCode::M,
                addr: 100
            }
        );
        assert_eq!(ret.value, 0x2030);
    }
    #[test]
    fn test_pack_bit2dword() {
        let mut src = [SLMPDeviceData::<bool> {
            dev: SLMPDevice {
                d_code: SLMPDeviceCode::M,
                addr: 1111,
            },
            value: true,
        }; 32];
        let buf = [
            true, false, true, false, true, true, true, false, false, false, true, false, false,
            false, false, false, true, false, true, false, false, true, false, false, false, false,
            true, false, false, false, false, false,
        ];
        for i in 0..32 {
            src[i].value = buf[i];
            src[i].dev.addr = 1111 + i as u32;
        }
        let ret = pack_bit2dword(&src);
        assert_eq!(
            ret.dev,
            SLMPDevice {
                d_code: SLMPDeviceCode::M,
                addr: 1111
            }
        );
        assert_eq!(ret.value, 0x04250475);
    }
}
/// ビットデータをビット単位でパック
///
/// # 引数
///
/// * `data` - ビットデータ
///
/// # 返値
///
/// パックしたバイト列
///
/// # 例
///
/// ```
/// use slmp_client::*;
/// let bit_data = [true, true, false, true, false, false];
/// let packed_data = pack_bits_by_bit(&bit_data);
/// assert_eq!(packed_data, vec![0x11u8, 0x01u8, 0x00u8]);
/// ```
pub fn pack_bits_by_bit(data: &[bool]) -> Vec<u8> {
    let mut buf = Vec::new();
    for (i, d) in data.iter().enumerate() {
        match i % 2 {
            0 => match d {
                true => buf.push(16u8),
                false => buf.push(0u8),
            },
            1 => {
                let tmp = buf.pop().unwrap();
                match d {
                    true => buf.push(tmp + 1u8),
                    false => buf.push(tmp),
                }
            }
            _ => unreachable!(),
        }
    }
    buf
}

/// ビットデータをワード単位でパック
///
/// # 引数
///
/// * `data` - ビットデータ
///
/// # 返値
///
/// パックしたバイト列
///
/// # 例
///
/// ```
/// use slmp_client::*;
/// let bit_data = [true, true, false, true, false, false];
/// let packed_data = pack_bits_by_word(&bit_data);
/// assert_eq!(packed_data, vec![0b00001011u8,]);
/// ```
pub fn pack_bits_by_word(data: &[bool]) -> Vec<u8> {
    let mut buf = Vec::new();
    for (i, d) in data.iter().enumerate() {
        if (i % 8) == 0 {
            buf.push(0u8);
        }
        let mut tmp = buf.pop().unwrap();
        if let true = d {
            tmp += 1u8 << (i % 8);
        }
        buf.push(tmp);
    }
    buf
}

/// ワードデータをワード単位でパック
///
/// # 引数
///
/// * `data` - ワード単位のデータ
///
/// # 返値
///
/// パックしたバイト列
///
/// # 例
///
/// ```
/// use slmp_client::*;
/// let raw_data = [0x1995u16, 0x1202u16, 0x1130u16];
/// let packed_data = pack_words_by_word(&raw_data);
/// assert_eq!(packed_data, vec![0x95, 0x19, 0x02, 0x12, 0x30, 0x11]);
/// ```
pub fn pack_words_by_word(data: &[u16]) -> Vec<u8> {
    let mut buf = Vec::new();
    for d in data.iter() {
        buf.push(*d as u8);
        buf.push((*d >> 8) as u8);
    }
    buf
}

/// ビット単位にパックされたビットデータを分解
///
/// # 引数
///
/// * `data` - ビット単位にパックされたビットデータ
///
/// # 返値
///
/// ビットデータ
///
/// # 例
///
/// ```
/// use slmp_client::*;
/// let raw_data = [0x00, 0x01, 0x00, 0x11];
/// let unpacked = unpack_bits_by_bit(&raw_data).unwrap();
/// assert_eq!(unpacked, vec![false, false, false, true, false, false, true, true]);
/// ```
pub fn unpack_bits_by_bit(data: &[u8]) -> Result<Vec<bool>, &'static str> {
    let mut buf = Vec::new();
    for d in data.iter() {
        match d {
            0x00 => {
                buf.push(false);
                buf.push(false);
            }
            0x01 => {
                buf.push(false);
                buf.push(true);
            }
            0x10 => {
                buf.push(true);
                buf.push(false);
            }
            0x11 => {
                buf.push(true);
                buf.push(true);
            }
            _ => return Err("Invalid Input Data"),
        }
    }
    Ok(buf)
}

/// ワード単位にパックされたビットデータを分解
///
/// # 引数
///
/// * `data` - ワード単位にパックされたビットデータ
///
/// # 返値
///
/// ビットデータ
///
/// # 例
///
/// ```
/// use slmp_client::*;
/// let raw_data = [0x34, 0x12];
/// let unpacked = unpack_bits_by_word(&raw_data);
/// assert_eq!(unpacked,
///    vec![false, false, true, false, true, true, false, false,
///         false, true, false, false, true, false, false, false]);
/// ```
pub fn unpack_bits_by_word(data: &[u8]) -> Vec<bool> {
    let mut buf = Vec::new();
    for d in data.iter() {
        for i in 0..8 {
            let tmp = 1u8 << i;
            if (d & tmp) == tmp {
                buf.push(true);
            } else {
                buf.push(false);
            }
        }
    }
    buf
}

/// ワード単位にパックされたワードデータを分解
///
/// # 引数
///
/// * `data` - ワード単位にパックされたワードデータ
///
/// # 返値
///
/// ワードデータ
///
/// # 例
///
/// ```
/// use slmp_client::*;
/// let raw_data = [0x34, 0x12, 0x02, 0x00, 0xef, 0x1d];
/// let unpacked = unpack_words_by_word(&raw_data);
/// assert_eq!(unpacked, vec![0x1234, 0x0002, 0x1def])
/// ```
pub fn unpack_words_by_word(data: &[u8]) -> Vec<u16> {
    let mut buf = Vec::new();
    for (i, d) in data.iter().enumerate() {
        if i % 2 == 0 {
            buf.push(*d as u16);
        } else {
            let tmp = buf.pop().unwrap();
            buf.push(((*d as u16) << 8) + tmp);
        }
    }
    buf
}

/// ダブルワード単位にパックされたダブルワードデータを分解
///
/// # 引数
///
/// * `data` - ダブルワード単位にパックされたダブルワードデータ
///
/// # 返値
///
/// ダブルワードデータ
///
/// # 例
///
/// ```
/// use slmp_client::*;
/// let raw_data = [0x4e, 0x4f, 0x54, 0x4c, 0xaf, 0xb9, 0xde, 0xc3];
/// let unpacked = unpack_dwords_by_dword(&raw_data);
/// assert_eq!(unpacked, vec![0x4c544f4e, 0xc3deb9af]);
/// ```
pub fn unpack_dwords_by_dword(data: &[u8]) -> Vec<u32> {
    let mut buf = Vec::new();
    for (i, d) in data.iter().enumerate() {
        let a = i % 4;
        let d = *d as u32;
        match a {
            0 => buf.push(d),
            1 => {
                let tmp = buf.pop().unwrap();
                buf.push(tmp + (d << 8));
            }
            2 => {
                let tmp = buf.pop().unwrap();
                buf.push(tmp + (d << 16));
            }
            3 => {
                let tmp = buf.pop().unwrap();
                buf.push(tmp + (d << 24));
            }
            _ => unreachable!("{}", a),
        }
    }
    buf
}

/// バイトにパックされたビットデータを分解
///
/// # 引数
///
/// * `data` - バイトにパックされたビットデータ
///
/// # 返値
///
/// ビットデータ
///
/// # 例
///
/// ```
/// use slmp_client::*;
/// let raw_data: u8 = 0x49;
/// let unpacked = unpack_bits_in_byte(raw_data);
/// assert_eq!(unpacked, [true, false, false, true, false, false, true, false]);
/// ```
pub fn unpack_bits_in_byte(data: u8) -> [bool; 8] {
    let mut buf = [false; 8];
    for i in 0..8 {
        let tmp = 1u8 << i;
        if data & tmp == tmp {
            buf[i] = true;
        }
    }
    buf
}

/// ワードにパックされたビットデータを分解
///
/// # 引数
///
/// * `data` - ワードにパックされたビットデータ
///
/// # 返値
///
/// ビットデータ
///
/// # 例
///
/// ```
/// use slmp_client::*;
/// let raw_data: u16 = 0x2030;
/// let unpacked = unpack_bits_in_word(raw_data);
/// assert_eq!(unpacked, [false, false, false, false, true, true, false, false,
///                           false, false, false, false, false, true, false, false]);
/// ```
pub fn unpack_bits_in_word(data: u16) -> [bool; 16] {
    let mut buf = [false; 16];
    let lows = unpack_bits_in_byte(data as u8);
    let highs = unpack_bits_in_byte((data >> 8) as u8);
    for (i, d) in lows.iter().enumerate() {
        buf[i] = *d;
    }
    for (i, d) in highs.iter().enumerate() {
        buf[i + 8] = *d;
    }
    buf
}

/// ダブルワードにパックされたビットデータを分解
///
/// # 引数
///
/// * `data` - ダブルワードにパックされたビットデータ
///
/// # 返値
///
/// ビットデータ
///
/// # 例
///
/// ```
/// use slmp_client::*;
/// let raw_data: u32 = 0xc3deb9af;
/// let packed = unpack_bits_in_dword(raw_data);
/// assert_eq!(packed, [
/// true, true, true, true, false, true, false, true,
/// true, false, false, true, true, true, false, true,
/// false, true, true, true, true, false, true, true,
/// true, true, false, false, false, false, true, true]);
/// ```
pub fn unpack_bits_in_dword(data: u32) -> [bool; 32] {
    let mut buf = [false; 32];
    let d1 = unpack_bits_in_byte((data >> 24) as u8);
    let d2 = unpack_bits_in_byte((data >> 16) as u8);
    let d3 = unpack_bits_in_byte((data >> 8) as u8);
    let d4 = unpack_bits_in_byte(data as u8);

    for (i, d) in d4.iter().enumerate() {
        buf[i] = *d;
    }
    for (i, d) in d3.iter().enumerate() {
        buf[i + 8] = *d;
    }
    for (i, d) in d2.iter().enumerate() {
        buf[i + 16] = *d;
    }
    for (i, d) in d1.iter().enumerate() {
        buf[i + 24] = *d;
    }
    buf
}

/// SLMPにおけるデバイス
#[derive(Copy, Clone, Debug, Hash)]
pub struct SLMPDevice {
    /// デバイス種別
    pub d_code: SLMPDeviceCode,
    /// アドレス
    pub addr: u32,
}
impl SLMPDevice {
    pub fn pack32(&self) -> [u8; 6] {
        let mut buf = [0u8; 6];
        buf[0] = self.addr as u8;
        buf[1] = (self.addr >> 8) as u8;
        buf[2] = (self.addr >> 16) as u8;
        buf[3] = (self.addr >> 24) as u8;
        buf[4] = self.d_code as u8;
        buf[5] = ((self.d_code as u16) >> 8) as u8;
        buf
    }
    pub fn pack16(&self) -> [u8; 4] {
        let mut buf = [0u8; 4];
        buf[0] = self.addr as u8;
        buf[1] = (self.addr >> 8) as u8;
        buf[2] = (self.addr >> 16) as u8;
        buf[3] = self.d_code as u8;
        buf
    }
}
impl PartialEq for SLMPDevice {
    fn eq(&self, other: &Self) -> bool {
        if self.d_code == other.d_code && self.addr == other.addr {
            true
        } else {
            false
        }
    }
}

/// SLMPにおけるデバイスの値
#[derive(Copy, Clone, Debug, Hash)]
pub struct SLMPDeviceData<T: PartialEq + Copy + Clone> {
    /// 対象デバイス
    pub dev: SLMPDevice,
    /// デバイスの持っている値
    pub value: T,
}
impl<T: PartialEq + Copy> PartialEq for SLMPDeviceData<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.dev == other.dev && self.value == other.value {
            true
        } else {
            false
        }
    }
}

impl SLMPDeviceData<bool> {
    pub fn pack32(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        let dev = self.dev.pack32();
        for i in 0..6 {
            buf[i] = dev[i];
        }
        if self.value {
            buf[7] = 1;
        }
        buf
    }
    pub fn pack16(&self) -> [u8; 5] {
        let mut buf = [0u8; 5];
        let dev = self.dev.pack16();
        for i in 0..4 {
            buf[i] = dev[i];
        }
        if self.value {
            buf[4] = 1;
        }
        buf
    }
}

impl SLMPDeviceData<u16> {
    pub fn pack32(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        let dev = self.dev.pack32();
        for i in 0..6 {
            buf[i] = dev[i];
        }
        buf[6] = self.value as u8;
        buf[7] = (self.value >> 8) as u8;
        buf
    }
    pub fn pack16(&self) -> [u8; 6] {
        let mut buf = [0u8; 6];
        let dev = self.dev.pack16();
        for i in 0..4 {
            buf[i] = dev[i];
        }
        buf[4] = self.value as u8;
        buf[5] = (self.value >> 8) as u8;
        buf
    }
    pub fn unpack2bits(&self) -> [SLMPDeviceData<bool>; 16] {
        unpack_word2bit(self)
    }
}

impl SLMPDeviceData<u32> {
    pub fn pack32(&self) -> [u8; 10] {
        let mut buf = [0u8; 10];
        let dev = self.dev.pack32();
        for i in 0..6 {
            buf[i] = dev[i];
        }
        buf[6] = self.value as u8;
        buf[7] = (self.value >> 8) as u8;
        buf[8] = (self.value >> 16) as u8;
        buf[9] = (self.value >> 24) as u8;
        buf
    }
    pub fn pack16(&self) -> [u8; 8] {
        let mut buf = [0u8; 8];
        let dev = self.dev.pack16();
        for i in 0..4 {
            buf[i] = dev[i];
        }
        buf[4] = self.value as u8;
        buf[5] = (self.value >> 8) as u8;
        buf[6] = (self.value >> 16) as u8;
        buf[7] = (self.value >> 24) as u8;
        buf
    }
    pub fn unpack2words(&self) -> [SLMPDeviceData<u16>; 2] {
        unpack_dword2word(self)
    }
    pub fn unpack2bits(&self) -> [SLMPDeviceData<bool>; 32] {
        unpack_dword2bit(self)
    }
}

fn unpack_word2bit(src: &SLMPDeviceData<u16>) -> [SLMPDeviceData<bool>; 16] {
    let mut ret = [SLMPDeviceData::<bool> {
        dev: SLMPDevice {
            d_code: src.dev.d_code,
            addr: src.dev.addr,
        },
        value: false,
    }; 16];
    let buf = unpack_bits_in_word(src.value);
    for i in 0..16 {
        ret[i].dev.addr += i as u32;
        ret[i].value = buf[i];
    }
    ret
}

fn unpack_dword2bit(src: &SLMPDeviceData<u32>) -> [SLMPDeviceData<bool>; 32] {
    let mut ret = [SLMPDeviceData::<bool> {
        dev: SLMPDevice {
            d_code: src.dev.d_code,
            addr: src.dev.addr,
        },
        value: false,
    }; 32];
    let buf = unpack_bits_in_dword(src.value);
    for i in 0..32 {
        ret[i].dev.addr += i as u32;
        ret[i].value = buf[i];
    }
    ret
}

/// 一連のビットデバイスデータをワードデバイスデータとする
/// # 引数
/// * `src` - 16個の連続したビットデバイスデータ
/// # 返値
/// 単一のワードデバイスデータ
pub fn pack_bit2word(src: &[SLMPDeviceData<bool>; 16]) -> SLMPDeviceData<u16> {
    let mut ret = SLMPDeviceData::<u16> {
        dev: SLMPDevice {
            d_code: src[0].dev.d_code,
            addr: src[0].dev.addr,
        },
        value: 0,
    };
    let mut buf = [false; 16];
    for i in 0..16 {
        buf[i] = src[i].value;
        assert_eq!(src[i].dev.addr, src[0].dev.addr + i as u32);
        assert_eq!(src[i].dev.d_code, src[0].dev.d_code);
    }
    let buf = pack_bits_by_word(&buf);
    ret.value = buf[0] as u16 + ((buf[1] as u16) << 8);

    ret
}

/// 一連のビットデバイスデータをダブルワードデバイスデータとする
/// # 引数
/// * `src` - 32個の連続したビットデバイスデータ
/// # 返値
/// 単一のダブルワードデバイスデータ
pub fn pack_bit2dword(src: &[SLMPDeviceData<bool>; 32]) -> SLMPDeviceData<u32> {
    let mut ret = SLMPDeviceData::<u32> {
        dev: SLMPDevice {
            d_code: src[0].dev.d_code,
            addr: src[0].dev.addr,
        },
        value: 0,
    };
    let mut buf = [false; 32];
    for i in 0..32 {
        buf[i] = src[i].value;
        assert_eq!(src[i].dev.addr, src[0].dev.addr + i as u32);
        assert_eq!(src[i].dev.d_code, src[0].dev.d_code);
    }
    let buf = pack_bits_by_word(&buf);
    ret.value =
        buf[0] as u32 + ((buf[1] as u32) << 8) + ((buf[2] as u32) << 16) + ((buf[3] as u32) << 24);
    ret
}

fn unpack_dword2word(src: &SLMPDeviceData<u32>) -> [SLMPDeviceData<u16>; 2] {
    let mut ret = [
        SLMPDeviceData::<u16> {
            dev: SLMPDevice {
                d_code: src.dev.d_code,
                addr: src.dev.addr,
            },
            value: 0,
        },
        SLMPDeviceData::<u16> {
            dev: SLMPDevice {
                d_code: src.dev.d_code,
                addr: src.dev.addr + 1,
            },
            value: 0,
        },
    ];
    ret[0].value = (src.value >> 16) as u16;
    ret[1].value = src.value as u16;
    ret
}
/// デバイスブロック
///
/// 先頭デバイスと点数で表される
///
/// ここで点数はビットデバイスであろうとワード単位となる
#[derive(Copy, Clone, Debug, Hash)]
pub struct SLMPDeviceBlock {
    /// 先頭デバイス
    pub top_device: SLMPDevice,
    /// デバイス点数（ワード単位）
    pub count: u16,
}
impl SLMPDeviceBlock {
    pub fn pack16(&self) -> [u8; 6] {
        let packed = self.top_device.pack16();
        let mut buf = [0; 6];
        for i in 0..4 {
            buf[i] = packed[i];
        }
        buf[4] = self.count as u8;
        buf[5] = (self.count >> 8) as u8;

        buf
    }
    pub fn pack32(&self) -> [u8; 8] {
        let packed = self.top_device.pack32();
        let mut buf = [0u8; 8];
        for i in 0..6 {
            buf[i] = packed[i];
        }
        buf[6] = self.count as u8;
        buf[7] = (self.count >> 8) as u8;
        buf
    }
}
impl PartialEq for SLMPDeviceBlock {
    fn eq(&self, other: &Self) -> bool {
        if self.top_device == other.top_device && self.count == other.count {
            true
        } else {
            false
        }
    }
}
impl PartialOrd for SLMPDevice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.d_code == other.d_code {
            if self.addr == other.addr {
                Some(Ordering::Equal)
            } else if self.addr < other.addr {
                Some(Ordering::Less)
            } else {
                Some(Ordering::Greater)
            }
        } else {
            None
        }
    }
}
impl PartialOrd for SLMPDeviceData<bool> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dev.partial_cmp(&other.dev)
    }
}
impl PartialOrd for SLMPDeviceData<u16> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dev.partial_cmp(&other.dev)
    }
}
impl PartialOrd for SLMPDeviceData<u32> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.dev.partial_cmp(&other.dev)
    }
}
impl PartialOrd for SLMPDeviceBlock {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.top_device.partial_cmp(&other.top_device)
    }
}

/// データを含んだデバイスブロック
#[derive(Clone, Debug, Hash)]
pub struct SLMPDeviceBlockData<T: PartialEq + Copy + Clone> {
    pub device_block: SLMPDeviceBlock,
    value: Vec<T>,
}
impl SLMPDeviceBlockData<bool> {
    /// データを含むデバイスブロックを新規作成
    ///
    /// # 引数
    ///
    /// * `top_device` - 先頭デバイス
    /// * `count` - 点数(ワード単位)
    pub fn new(top_device: SLMPDevice, count: u16) -> SLMPDeviceBlockData<bool> {
        let mut ret = SLMPDeviceBlockData::<bool> {
            device_block: SLMPDeviceBlock { top_device, count },
            value: Vec::new(),
        };
        for _ in 0..count {
            for _ in 0..16 {
                ret.value.push(false);
            }
        }
        ret
    }
    pub fn sets(&mut self, buf: &[bool]) {
        self.value.copy_from_slice(buf);
    }
    pub fn set(&mut self, dd: SLMPDeviceData<bool>) -> Result<(), &'static str> {
        if dd.dev.d_code != self.device_block.top_device.d_code {
            return Err("Different device");
        }
        if dd.dev.addr < self.device_block.top_device.addr {
            return Err("Address Not in range");
        }
        let addr_max =
            self.device_block.top_device.addr + (self.device_block.count as u32) * 16 - 1;
        if dd.dev.addr > addr_max {
            return Err("Address Not in range");
        }
        let d_addr = (dd.dev.addr - self.device_block.top_device.addr) as usize;
        self.value[d_addr] = dd.value;
        Ok(())
    }
    pub fn gets(&self) -> Vec<SLMPDeviceData<bool>> {
        let mut ret = Vec::new();
        for i in 0..self.device_block.count {
            let d_tmp = SLMPDeviceData::<bool> {
                dev: SLMPDevice {
                    d_code: self.device_block.top_device.d_code,
                    addr: self.device_block.top_device.addr + i as u32,
                },
                value: self.value[i as usize],
            };
            ret.push(d_tmp);
        }
        ret
    }
    pub fn get(&self, dev: SLMPDevice) -> Result<SLMPDeviceData<bool>, &'static str> {
        if dev.d_code != self.device_block.top_device.d_code {
            return Err("Different device");
        }
        if dev.addr < self.device_block.top_device.addr {
            return Err("Address Not in range");
        }
        let addr_max =
            self.device_block.top_device.addr + (self.device_block.count as u32) * 16 - 1;
        if dev.addr > addr_max {
            return Err("Address Not in range");
        }
        let d_addr = (dev.addr - self.device_block.top_device.addr) as usize;
        Ok(SLMPDeviceData::<bool> {
            dev,
            value: self.value[d_addr],
        })
    }
    pub fn pack16(&self) -> Vec<u8> {
        let mut buf = Vec::from(self.device_block.pack16());
        let packed = pack_bits_by_word(&self.value);
        buf.extend_from_slice(&packed);
        buf
    }
    pub fn pack32(&self) -> Vec<u8> {
        let mut buf = Vec::from(self.device_block.pack32());
        let packed = pack_bits_by_word(&self.value);
        buf.extend_from_slice(&packed);
        buf
    }
    pub fn decode(&mut self, buf: &[u8]) {
        let data = unpack_bits_by_word(buf);
        self.sets(&data);
    }
}
impl SLMPDeviceBlockData<u16> {
    pub fn new(top_device: SLMPDevice, count: u16) -> SLMPDeviceBlockData<u16> {
        let mut ret = SLMPDeviceBlockData::<u16> {
            device_block: SLMPDeviceBlock { top_device, count },
            value: Vec::new(),
        };
        for _ in 0..count {
            ret.value.push(0u16);
        }
        ret
    }
    pub fn sets(&mut self, buf: &[u16]) {
        self.value.copy_from_slice(buf);
    }
    pub fn set(&mut self, dd: SLMPDeviceData<u16>) -> Result<(), &'static str> {
        if dd.dev.d_code != self.device_block.top_device.d_code {
            return Err("Different device");
        }
        if dd.dev.addr < self.device_block.top_device.addr {
            return Err("Address Not in range");
        }
        let addr_max = self.device_block.top_device.addr + (self.device_block.count as u32) - 1;
        if dd.dev.addr > addr_max {
            return Err("Address Not in range");
        }
        let d_addr = (dd.dev.addr - self.device_block.top_device.addr) as usize;
        self.value[d_addr] = dd.value;
        Ok(())
    }
    pub fn gets(&self) -> Vec<SLMPDeviceData<u16>> {
        let mut ret = Vec::new();
        for i in 0..self.device_block.count {
            let d_tmp = SLMPDeviceData::<u16> {
                dev: SLMPDevice {
                    d_code: self.device_block.top_device.d_code,
                    addr: self.device_block.top_device.addr + i as u32,
                },
                value: self.value[i as usize],
            };
            ret.push(d_tmp);
        }
        ret
    }
    pub fn get(&self, dev: SLMPDevice) -> Result<SLMPDeviceData<u16>, &'static str> {
        if dev.d_code != self.device_block.top_device.d_code {
            return Err("Different device");
        }
        if dev.addr < self.device_block.top_device.addr {
            return Err("Address Not in range");
        }
        let addr_max =
            self.device_block.top_device.addr + (self.device_block.count as u32) * 16 - 1;
        if dev.addr > addr_max {
            return Err("Address Not in range");
        }
        let d_addr = (dev.addr - self.device_block.top_device.addr) as usize;
        Ok(SLMPDeviceData::<u16> {
            dev,
            value: self.value[d_addr],
        })
    }
    pub fn pack16(&self) -> Vec<u8> {
        let mut buf = Vec::from(self.device_block.pack16());
        let packed = pack_words_by_word(&self.value);
        buf.extend_from_slice(&packed);
        buf
    }
    pub fn pack32(&self) -> Vec<u8> {
        let mut buf = Vec::from(self.device_block.pack32());
        let packed = pack_words_by_word(&self.value);
        buf.extend_from_slice(&packed);
        buf
    }
    pub fn decode(&mut self, buf: &[u8]) {
        let data = unpack_words_by_word(buf);
        self.sets(&data);
    }
}
