mod dual_port_memory;
mod enums;
mod internal_memory;
mod packing;
mod remote_control;
mod structs;

pub use dual_port_memory::*;
pub use enums::*;
pub use internal_memory::*;
pub use packing::*;
pub use remote_control::*;
pub use structs::*;

/// デバイス読み書き時のデバイス指定32bit版
///
/// # 引数
///
/// * `d_type` - デバイスタイプ
/// * `first_addr` - 先頭アドレス
/// * `count` - 点数
///
/// # 返値
///
/// パックしたバイト列
pub fn make_cmd_rw_devices32(d_type: SLMPDeviceCode, first_addr: u32, count: u16) -> Vec<u8> {
    let mut buf = Vec::new();
    let d_code = d_type as u16;
    buf.push(d_code as u8);
    buf.push((d_code >> 8) as u8);
    buf.push(first_addr as u8);
    buf.push((first_addr >> 8) as u8);
    buf.push((first_addr >> 16) as u8);
    buf.push((first_addr >> 24) as u8);
    buf.push(count as u8);
    buf.push((count >> 8) as u8);
    buf
}

/// デバイス読み書き時のデバイス指定16bit版
///
/// # 引数
///
/// * `d_type` - デバイスタイプ
/// * `first_addr` - 先頭アドレス
/// * `count` - 点数
///
/// # 返値
///
/// パックしたバイト列
pub fn make_cmd_rw_devices16(d_type: SLMPDeviceCode, first_addr: u16, count: u16) -> Vec<u8> {
    let mut buf = Vec::new();
    let d_code = d_type as u8;
    buf.push(d_code);
    buf.push(first_addr as u8);
    buf.push((first_addr >> 8) as u8);
    buf.push(0u8);
    buf.push(count as u8);
    buf.push((count >> 8) as u8);
    buf
}
/// セルフチェック
///
/// # 引数
///
/// * `connection_info` - 接続情報
/// * `timeout` - SLMPコマンドのタイムアウト
/// * `data` - 折り返しチェックのための文字列データ、0-9もしくはA-Zのみが使用可能
///
/// # 返値
///
/// 送信時のシリアル
pub fn send_self_test_cmd(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    data: &[u8],
) -> Option<u16> {
    let mut buf = Vec::new();
    let length = data.len();
    if length > 960 {
        eprintln!("too long data");
        return None;
    }
    buf.push(length as u8);
    buf.push((length >> 8) as u8);
    buf.extend_from_slice(data);
    connection_info.send_cmd(timeout, SLMPCommand::SelfTest, 0, &buf)
}
/// セルフテストの応答を処理
/// # 引数
/// * `buf` - セルフテストの応答を含んだバッファ
/// # 返値
/// セルフテストで送信したデータ
pub fn decode_self_test_response(buf: &[u8]) -> Vec<u8> {
    let mut ret = Vec::new();
    for d in buf[2..].iter() {
        ret.push(*d);
    }
    ret
}

/// エラークリア
/// # 引数
/// * `connection_info` - SLMP接続情報
/// * `timeout` - SLMPコマンドのタイムアウト, 250msec単位
/// # 返値
/// コマンド発行時のシリアル番号
pub fn send_clear_error_cmd(connection_info: &mut SLMPConnectionInfo, timeout: u16) -> Option<u16> {
    connection_info.send_cmd(timeout, SLMPCommand::ClearErrorCode, 0, &[])
}
/// 受信したオンデマンドデータの処理
/// # 引数
/// * `buf` - 受信したオンデマンドデータの入ったバッファ
/// # 返値
/// オンデマンドデータ
pub fn decode_on_demand_data(buf: &[u8]) -> Option<Vec<u8>> {
    if buf[0] != 1 || buf[1] != 0x21 || buf[2] != 0 || buf[3] != 0 {
        None
    } else {
        let mut ret = Vec::new();
        ret.copy_from_slice(&buf[3..]);
        Some(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[test]
    #[ignore]
    pub fn test_udp_self_test() {
        let mut connection_info = SLMPConnectionInfo::new_udp(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(192, 168, 1, 10)),
            5000,
        ));
        let buf = [0x31, 0x32, 0x34];
        let seq = send_self_test_cmd(&mut connection_info, 40, &buf);
        let (seq_r, buf_r, end_code) = connection_info.recv_cmd();
        assert_eq!(seq.unwrap(), seq_r);
        assert_eq!(end_code, Some(SLMPEndCode::Success));
        let ret = decode_self_test_response(&buf_r);
        assert_eq!(ret.len(), 3);
        assert_eq!(ret[0] as u8, buf[0] as u8);
        assert_eq!(ret[1] as u8, buf[1] as u8);
        assert_eq!(ret[2] as u8, buf[2] as u8);
    }
}
