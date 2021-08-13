use crate::enums::SLMPCommand;
use crate::SLMPConnectionInfo;

/// 自局のデュアルポートメモリのデータに対する読み取り要求の送信
/// # 引数
/// * `connection_info` - SLMP接続情報
/// * `timeout` - SLMPコマンドのタイムアウト
/// * `start_addr` - 先頭アドレス
/// * `word_length` - ワード長
/// # 返値
/// 発行したコマンドのシリアル
pub fn send_read_memory(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    start_addr: u32,
    word_length: u16,
) -> Option<u16> {
    let mut buf = [0u8; 6];
    buf[0] = start_addr as u8;
    buf[1] = (start_addr >> 8) as u8;
    buf[2] = (start_addr >> 16) as u8;
    buf[3] = (start_addr >> 24) as u8;
    if word_length == 0 || word_length > 480 {
        return None;
    }
    buf[4] = word_length as u8;
    buf[5] = (word_length >> 8) as u8;

    connection_info.send_cmd(timeout, SLMPCommand::MemoryRead, 0x00, &buf)
}

/// 自局のデュアルポートメモリないし拡張ユニットのメモリのデータに対する読み取り要求に対する応答の処理
/// # 引数
/// * `buf` - 応答内容の入ったバッファ
/// # 返り値
/// 読み取ったメモリの内容、もしくはエラー内容を含んだ文字列
pub fn decode_read_memory(buf: &[u8]) -> Result<Vec<u16>, &'static str> {
    let mut ret = Vec::new();
    let buf_length = buf.len();
    let mut i = 0;
    if buf_length < 2 && (buf_length % 2 != 0) {
        return Err("Few length buf");
    }
    while i < buf_length {
        ret.push((buf[i] as u16) + ((buf[i + 1] as u16) << 8));
        i += 2;
    }
    Ok(ret)
}

/// 自局のデュアルポートメモリのデータに対する書き込み要求の送信
/// # 引数
/// * `connection_info` - SLMP接続情報
/// * `timeout` - SLMPコマンドのタイムアウト
/// * `start_addr` - 先頭アドレス
/// * `word_length` - ワード長
/// * `data` - 書き込みデータ
/// # 返値
/// 発行したコマンドのシリアル
pub fn send_write_memory(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    start_addr: u32,
    word_length: u16,
    data: &[u16],
) -> Option<u16> {
    let mut buf = Vec::new();
    buf.push(start_addr as u8);
    buf.push((start_addr >> 8) as u8);
    buf.push((start_addr >> 16) as u8);
    buf.push((start_addr >> 24) as u8);
    if word_length == 0 || word_length > 480 {
        return None;
    }
    buf.push(word_length as u8);
    buf.push((word_length >> 8) as u8);
    if (word_length as usize) != data.len() {
        return None;
    }
    for datum in data {
        buf.push(*datum as u8);
        buf.push((*datum >> 8) as u8);
    }

    connection_info.send_cmd(timeout, SLMPCommand::MemoryWrite, 0x00, &buf)
}

/// 拡張ユニットのメモリに対してデータの読み取り要求の送信
///
/// 応答は[decode_read_memory]で処理する
///
/// # 引数
/// * `connection_info` - SLMP接続情報
/// * `timeout` - SLMPコマンドのタイムアウト
/// * `start_addr` - 先頭アドレス
/// * `byte_length` - バイト数、アドレス単位がワード(2バイト)であることに注意
/// * `unit_no` - ユニット番号
/// # 返値
/// 発行したコマンドのシリアル
///
pub fn send_read_extend_unit_memory(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    start_addr: u32,
    byte_length: u16,
    unit_no: u16,
) -> Option<u16> {
    let mut buf = [0; 8];
    buf[0] = start_addr as u8;
    buf[1] = (start_addr >> 8) as u8;
    buf[2] = (start_addr >> 16) as u8;
    buf[3] = (start_addr >> 24) as u8;
    if byte_length < 2 || byte_length > 1920 {
        return None;
    }
    buf[4] = byte_length as u8;
    buf[5] = (byte_length >> 8) as u8;
    buf[6] = unit_no as u8;
    buf[7] = (unit_no >> 8) as u8;

    connection_info.send_cmd(timeout, SLMPCommand::ExtendUnitRead, 0x00, &buf)
}

/// 拡張ユニットのメモリに対してデータの書き込み要求の送信
/// # 引数
/// * `connection_info` - SLMP接続情報
/// * `timeout` - SLMPコマンドのタイムアウト
/// * `start_addr` - 先頭アドレス
/// * `byte_length` - バイト数、アドレス単位がワード(2バイト)であることに注意
/// * `unit_no` - ユニット番号
/// * `data` - 書き込みデータ
/// # 返値
/// 発行したコマンドのシリアル
pub fn send_write_extend_unit_memory(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    start_addr: u32,
    byte_length: u16,
    unit_no: u16,
    data: &[u16],
) -> Option<u16> {
    let mut buf = Vec::new();
    buf.push(start_addr as u8);
    buf.push((start_addr >> 8) as u8);
    buf.push((start_addr >> 16) as u8);
    buf.push((start_addr >> 24) as u8);
    if byte_length == 0 || byte_length > 1920 {
        return None;
    }
    buf.push(byte_length as u8);
    buf.push((byte_length >> 8) as u8);
    buf.push(unit_no as u8);
    buf.push((unit_no >> 8) as u8);
    if (byte_length as usize) != data.len() * 2 {
        return None;
    }
    for datum in data {
        buf.push(*datum as u8);
        buf.push((*datum >> 8) as u8);
    }

    connection_info.send_cmd(timeout, SLMPCommand::ExtendUnitWrite, 0x00, &buf)
}

#[cfg(test)]
mod tests {
    use crate::dual_port_memory::decode_read_memory;
    #[test]
    pub fn test_decode_read_memory() {
        let buf = [0x00, 0x05, 0xc1, 0x09, 0xc8, 0x00];
        let ret = decode_read_memory(&buf).unwrap();
        assert_eq!(ret.len(), 3);
        assert_eq!(ret[0], 0x0500);
        assert_eq!(ret[1], 0x09c1);
        assert_eq!(ret[2], 0x00c8);
    }
}
