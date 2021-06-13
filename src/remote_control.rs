use crate::{SLMPClearMode, SLMPCommand, SLMPConnectionInfo};

/// リモートRUN要求を送信
/// # 引数
/// * `connection_info` - SLMP接続情報
/// * `timeout` - SLMPコマンドのタイムアウト
/// * `force` - 強制実行するかどうか
/// * `clear_mode` - クリアモード
/// # 返値
/// 発行したコマンドのシリアル
pub fn send_remote_run_cmd(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    force: bool,
    clear_mode: SLMPClearMode,
) -> Option<u16> {
    let mut buf = Vec::new();
    match force {
        true => buf.push(0x03),
        false => buf.push(0x01),
    }
    buf.push(0);
    buf.push(clear_mode as u8);
    buf.push(0);

    connection_info.send_cmd(timeout, SLMPCommand::RemoteRun, 0, &buf)
}

/// リモートSTOP要求の送信
/// # 引数
/// * `connection_info` - SLMP接続情報
/// * `timeout` - SLMPコマンドのタイムアウト
/// # 返値
/// 発行したコマンドのシリアル
pub fn send_remote_stop_cmd(connection_info: &mut SLMPConnectionInfo, timeout: u16) -> Option<u16> {
    connection_info.send_cmd(timeout, SLMPCommand::RemoteStop, 0, &[1u8, 0])
}
/// リモートPAUSE要求を送信
/// # 引数
/// * `connection_info` - SLMP接続情報
/// * `timeout` - SLMPコマンドのタイムアウト
/// * `force` - 強制実行するかどうか
/// # 返値
/// 発行したコマンドのシリアル
pub fn send_remote_pause_cmd(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    force: bool,
) -> Option<u16> {
    let buf = match force {
        true => [1, 0],
        false => [3, 0],
    };
    connection_info.send_cmd(timeout, SLMPCommand::RemotePause, 0, &buf)
}
/// リモートラッチクリア要求を送信
/// # 引数
/// * `connection_info` - SLMP接続情報
/// * `timeout` - SLMPコマンドのタイムアウト
/// # 返値
/// 発行したコマンドのシリアル
pub fn send_remote_latch_clear(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
) -> Option<u16> {
    connection_info.send_cmd(timeout, SLMPCommand::RemoteLatchClear, 0, &[1, 0])
}
/// リモートRESET要求を送信
/// # 引数
/// * `connection_info` - SLMP接続情報
/// * `timeout` - SLMPコマンドのタイムアウト
/// # 返値
/// 発行したコマンドのシリアル
pub fn send_remote_reset_cmd(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
) -> Option<u16> {
    connection_info.send_cmd(timeout, SLMPCommand::RemoteReset, 0, &[1, 0])
}
/// 形名要求を送信
/// # 引数
/// * `connection_info` - SLMP接続情報
/// * `timeout` - SLMPコマンドのタイムアウト
/// # 返値
/// 発行したコマンドのシリアル
pub fn send_read_type_name_cmd(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
) -> Option<u16> {
    connection_info.send_cmd(timeout, SLMPCommand::ReadTypeName, 0, &[])
}
/// 形名要求に対する応答の処理
/// # 引数
/// * `buf` - 応答内容の入ったバッファ
/// # 返値
/// 形名と形名コード
pub fn decode_read_type_name_response(buf: &[u8]) -> (String, u16) {
    let mut s = String::new();
    for i in 0..16 {
        s.push(buf[i] as char);
    }
    let code = buf[16] as u16 + (buf[17] as u16) << 8;
    (s, code)
}
