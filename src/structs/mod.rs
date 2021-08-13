mod headers;

use super::enums::SLMPCommand;
use super::enums::SLMPEndCode;
use headers::*;

/// エラー情報
pub struct ErrInfo {
    /// ネットワーク番号
    pub net_no: u8,
    /// 局番
    pub node_no: u8,
    /// IOユニット番号
    pub dst_proc_no: u16,
    /// 予約もしくはマルチドロップ番号
    pub reserved1: u8,
    /// 要求時のコマンド
    pub command: u16,
    /// 要求時のサブコマンド
    pub sub_command: u16,
}
impl ErrInfo {
    pub fn from(buf: [u8; 9]) -> ErrInfo {
        let net_no = buf[0];
        let node_no = buf[1];
        let dst_proc_no = (buf[2] as u16) + ((buf[3] as u16) << 8);
        let reserved1 = buf[4];
        let command = (buf[5] as u16) + ((buf[6] as u16) << 8);
        let sub_command = (buf[7] as u16) + ((buf[8] as u16) << 8);
        ErrInfo {
            net_no,
            node_no,
            dst_proc_no,
            reserved1,
            command,
            sub_command,
        }
    }
}

use crate::enums::TCPorUDP;
use std::io;
use std::net::{SocketAddr, TcpStream, UdpSocket};

/// SLMPの接続管理
pub struct SLMPConnectionInfo {
    /// ネットワーク番号
    network: u8,
    /// ノード番号
    node: u8,
    /// プロセッサ番号
    dst_proc: u16,
    /// マルチドロップ番号
    m_drop: u8,
    /// シリアル番号
    seq_no: u16,
    /// 通信ソケット
    socket: Option<TCPorUDP>,
    /// 受信バッファ
    buf: Vec<u8>,
}

use std::default::Default;
extern crate rand;
use rand::Rng;

impl Default for SLMPConnectionInfo {
    fn default() -> Self {
        let rnd: u8 = rand::thread_rng().gen();
        SLMPConnectionInfo {
            network: 1,
            node: 1,
            dst_proc: 0x3FF,
            m_drop: 0,
            seq_no: rnd as u16,
            socket: None,
            buf: Vec::new(),
        }
    }
}

impl SLMPConnectionInfo {
    /// TCPにてSLMP接続を作成
    ///
    /// # 引数
    ///
    /// * `addr` - 接続先
    pub fn new_tcp(addr: SocketAddr) -> SLMPConnectionInfo {
        let stream = TcpStream::connect(addr).unwrap();
        SLMPConnectionInfo {
            socket: Some(TCPorUDP::TCP(stream)),
            ..SLMPConnectionInfo::default()
        }
    }
    /// UDPにてSLMP接続を作成
    ///
    /// # 引数
    ///
    /// * `addr` - 接続先
    pub fn new_udp(addr: SocketAddr) -> SLMPConnectionInfo {
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        SLMPConnectionInfo {
            socket: Some(TCPorUDP::UDP(socket, addr)),
            ..SLMPConnectionInfo::default()
        }
    }
    pub fn get_network(&self) -> u8 {
        self.network
    }
    pub fn set_network(&mut self, network: u8) {
        self.network = network;
    }
    pub fn get_node(&self) -> u8 {
        self.node
    }
    pub fn set_node(&mut self, node: u8) {
        self.node = node
    }
    pub fn get_dst_proc(&self) -> u16 {
        self.dst_proc
    }
    pub fn set_dst_proc(&mut self, dst_proc: u16) {
        self.dst_proc = dst_proc
    }
    pub fn get_m_drop(&self) -> u8 {
        self.m_drop
    }
    pub fn set_m_drop(&mut self, m_drop: u8) {
        self.m_drop = m_drop
    }
    /// 指定したバイト列を送信する
    /// # 引数
    ///
    /// * `buf` - 送信するバイト列
    ///
    /// # 返値
    ///
    /// 送信したバイト数
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.socket.as_mut().unwrap().write(buf)
    }
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.socket.as_mut().unwrap().read(buf)
    }
    /// 新しい要求用のシリアル番号を取得する
    /// # 返値
    /// シリアル番号
    fn get_new_serial(&mut self) -> u16 {
        self.seq_no += 0x10;
        if self.seq_no > 0xff00 {
            let rnd: u8 = rand::thread_rng().gen();
            self.seq_no = rnd as u16;
        }
        self.seq_no
    }
    fn set_timeout(&mut self, timeout: u64) {
        self.socket.as_mut().unwrap().set_timeout(timeout);
    }
    /// SLMPの応答を受信
    ///
    ///  # 引数
    ///
    /// * `connection_info` - 接続情報
    ///
    /// # 返値
    ///
    /// シリアル番号と受信ペイロードと終了コード
    pub fn recv_cmd(&mut self) -> (u16, Vec<u8>, Option<SLMPEndCode>) {
        let ser_no;
        let mut req_data = Vec::new();
        let mut end_code: Option<SLMPEndCode> = None;
        let mut buf = [0 as u8; 8194];
        let mut count = 0;
        while self.buf.len() < 15 {
            if count > 3 {
                eprintln!("3times retry finish");
                return (0, req_data, end_code);
            }
            let recv_result = self.read(&mut buf);
            match recv_result {
                Ok(recv_size) => self.buf.extend_from_slice(&buf[..recv_size]),
                Err(e) => eprintln!("{}", e.to_string()),
            }
            count += 1;
        }
        match self.buf.remove(0) {
            // odReqMT-PDU
            0x54 => {
                assert_eq!(self.buf[0], 0x00);
                self.buf.remove(0);
                let ser_low = self.buf.remove(0) as u16;
                let ser_high = self.buf.remove(0) as u16;
                ser_no = ser_low + (ser_high << 8);
                // reserved
                self.buf.remove(0);
                self.buf.remove(0);
                // sub_header
                assert_eq!(self.buf[0], self.get_network());
                self.buf.remove(0); // netNo
                assert_eq!(self.buf[0], self.get_node());
                self.buf.remove(0); // nodeNo
                assert_eq!(
                    self.buf[0] as u16 + (self.buf[1] as u16) << 8,
                    self.get_dst_proc()
                );
                self.buf.remove(0);
                self.buf.remove(0); // dstProcNo
                assert_eq!(self.buf[0], self.get_m_drop());
                self.buf.remove(0); // reserved1
                let dl_low = self.buf.remove(0) as u16;
                let dl_high = self.buf.remove(0) as u16;
                let dl = (dl_low + (dl_high << 8)) - 6;
                // reserved3
                self.buf.remove(0);
                self.buf.remove(0);
                count = 0;
                while self.buf.len() < (dl + 4) as usize {
                    if count > 3 {
                        eprintln!("3times retry finish");
                        return (ser_no, req_data, end_code);
                    }
                    let recv_result = self.read(&mut buf);
                    match recv_result {
                        Ok(recv_size) => self.buf.extend_from_slice(&buf[..recv_size]),
                        Err(e) => eprintln!("{}", e.to_string()),
                    }
                    count += 1;
                }
                let cmd_low = self.buf.remove(0) as u16;
                let cmd_high = self.buf.remove(0) as u16;
                let cmd = cmd_low + (cmd_high << 8);
                assert_eq!(cmd, 0x2101);
                let sub_cmd_low = self.buf.remove(0) as u16;
                let sub_cmd_high = self.buf.remove(0) as u16;
                let sub_cmd = sub_cmd_low + (sub_cmd_high << 8);
                assert_eq!(sub_cmd, 0x0000);
                loop {
                    req_data.push(self.buf[0]);
                    self.buf.remove(0);
                    if req_data.len() == dl as usize {
                        break;
                    }
                }
            }
            // rdResMT-PDU, wrResMT-PDU, rdErrMT-PDU, wrErrMT-PDU
            0xD4 => {
                let buf_header = [
                    0xd4u8,
                    self.buf.remove(0),
                    self.buf.remove(0),
                    self.buf.remove(0),
                    self.buf.remove(0),
                    self.buf.remove(0),
                ];
                let sub_header = SlmpMTHeader::from(&buf_header);
                ser_no = sub_header.serial_no;
                let buf_target = [
                    self.buf.remove(0),
                    self.buf.remove(0),
                    self.buf.remove(0),
                    self.buf.remove(0),
                    self.buf.remove(0),
                    self.buf.remove(0),
                    self.buf.remove(0),
                    self.buf.remove(0),
                    self.buf.remove(0),
                ];
                let target = SlmpSubHeaderRes::from(&buf_target);
                let dl = target.dl - 2;
                end_code = SLMPEndCode::get(target.end_code);
                assert_eq!(target.net_no, self.network);
                assert_eq!(target.node_no, self.node);
                assert_eq!(target.dst_proc_no, self.dst_proc);
                count = 0;
                while self.buf.len() < dl as usize {
                    if count > 3 {
                        eprintln!("3times retry finish");
                        return (ser_no, req_data, end_code);
                    }
                    let recv_result = self.read(&mut buf);
                    match recv_result {
                        Ok(recv_size) => self.buf.extend_from_slice(&buf[..recv_size]),
                        Err(e) => eprintln!("{}", e.to_string()),
                    }
                    count += 1;
                }
                loop {
                    req_data.push(self.buf.remove(0));
                    if req_data.len() == dl as usize {
                        break;
                    }
                }
            }
            // 上記以外
            _ => {
                eprintln!("Wrong Data received");
                return (0, req_data, None);
            }
        };

        (ser_no, req_data, end_code)
    }
    /// SLMPコマンドを送信する
    ///
    /// # 引数
    ///
    /// * `connection_info` - SLMPの接続情報
    /// * `timeout` - SLMPコマンドのタイムアウト時間、単位は250ms
    /// * `cmd` - SLMPコマンド
    /// * `sub_command` - サブコマンド
    /// * `content_data` - コマンドごとの付属データ
    ///
    /// # 返値
    ///
    /// 送信したコマンドに対応するシリアル番号
    pub fn send_cmd(
        &mut self,
        timeout: u16,
        cmd: SLMPCommand,
        sub_command: u16,
        content_data: &[u8],
    ) -> Option<u16> {
        let mut seq_no = self.get_new_serial();
        let slmp_header = SlmpSubHeaderReq {
            net_no: self.network,
            node_no: self.node,
            dst_proc_no: self.dst_proc,
            reserved1: self.m_drop,
            dl: (content_data.len() + 4) as u16,
            timer: timeout,
        };
        self.set_timeout((timeout as u64) * 250);
        // 3回までリトライ
        for i in 0..3 {
            seq_no -= 1;
            let header = make_frame_header(&slmp_header, seq_no, cmd, sub_command);
            let mut write_buf: Vec<u8> = Vec::from(header);
            write_buf.extend_from_slice(&content_data);
            let send_result = self.write(&write_buf);
            match send_result {
                Ok(send_size) => {
                    if send_size != write_buf.len() {
                        eprintln!("write size error.");
                        if i == 2 {
                            return None;
                        }
                    } else {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("{}", e.to_string());
                    if i == 2 {
                        return None;
                    }
                }
            }
        }
        Some(seq_no)
    }
}
fn make_frame_header(
    slmp_header: &SlmpSubHeaderReq,
    seq_no: u16,
    command: SLMPCommand,
    sub_command: u16,
) -> [u8; 19] {
    let mut buf = [0u8; 19];
    let pre = slmp_header.to();
    // フレームタイプ0x0054
    buf[0] = 0x54;
    buf[1] = 0x00;
    // シリアル番号
    buf[2] = seq_no as u8;
    buf[3] = (seq_no >> 8) as u8;
    // 拡張用
    buf[4] = 0x00;
    buf[5] = 0x00;
    // ネットワーク番号
    // ノード番号
    // プロセッサ番号2byte
    // マルチドロップ
    // データ長2byte
    // タイマ2byte
    for i in 0..9 {
        buf[6 + i] = pre[i];
    }
    // コマンド2byte
    let command = command as u16;
    buf[15] = command as u8;
    buf[16] = (command >> 8) as u8;
    // サブコマンド2byte
    buf[17] = sub_command as u8;
    buf[18] = (sub_command >> 8) as u8;
    buf
}
