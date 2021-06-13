/// MTタイプの通信文でのヘッダ
pub struct SlmpMTHeader {
    /// フレームタイプ
    pub f_type: u16,
    /// シリアル番号
    pub serial_no: u16,
    /// 予約
    pub reserved2: u16,
}
impl SlmpMTHeader {
    pub fn from(buf: [u8; 6]) -> SlmpMTHeader {
        let f_type = (buf[0] as u16) + ((buf[1] as u16) << 8);
        let serial_no = (buf[2] as u16) + ((buf[3] as u16) << 8);
        let reserved2 = (buf[4] as u16) + ((buf[5] as u16) << 8);
        SlmpMTHeader {
            f_type,
            serial_no,
            reserved2,
        }
    }
}
/// 応答におけるサブヘッダ
pub struct SlmpSubHeaderRes {
    /// ネットワーク番号
    pub net_no: u8,
    /// 局番
    pub node_no: u8,
    /// IOユニット番号
    pub dst_proc_no: u16,
    /// 予約もしくはマルチドロップ番号
    pub reserved1: u8,
    /// データ長
    pub dl: u16,
    /// 終了コード
    pub end_code: u16,
}
impl SlmpSubHeaderRes {
    pub fn from(buf: [u8; 9]) -> SlmpSubHeaderRes {
        let net_no = buf[0];
        let node_no = buf[1];
        let dst_proc_no = (buf[2] as u16) + ((buf[3] as u16) << 8);
        let reserved1 = buf[4];
        let dl = (buf[5] as u16) + ((buf[6] as u16) << 8);
        let end_code = (buf[7] as u16) + ((buf[8] as u16) << 8);
        SlmpSubHeaderRes {
            net_no,
            node_no,
            dst_proc_no,
            reserved1,
            dl,
            end_code,
        }
    }
}
/// 要求におけるサブヘッダ
pub struct SlmpSubHeaderReq {
    /// ネットワーク番号
    pub net_no: u8,
    /// 局番
    pub node_no: u8,
    /// IOユニット番号
    pub dst_proc_no: u16,
    /// 予約もしくはマルチドロップ番号
    pub reserved1: u8,
    /// データ長
    pub dl: u16,
    /// 最大待機時間(250msec単位)
    pub timer: u16,
}
impl SlmpSubHeaderReq {
    pub fn from(buf: [u8; 9]) -> SlmpSubHeaderReq {
        let net_no = buf[0];
        let node_no = buf[1];
        let dst_proc_no = (buf[2] as u16) + ((buf[3] as u16) << 8);
        let reserved1 = buf[4];
        let dl = (buf[5] as u16) + ((buf[6] as u16) << 8);
        let timer = (buf[7] as u16) + ((buf[8] as u16) << 8);
        SlmpSubHeaderReq {
            net_no,
            node_no,
            dst_proc_no,
            reserved1,
            dl,
            timer,
        }
    }
    pub fn to(&self) -> [u8; 9] {
        let mut buf = [0; 9];
        buf[0] = self.net_no;
        buf[1] = self.node_no;
        buf[2] = self.dst_proc_no as u8;
        buf[3] = (self.dst_proc_no >> 8) as u8;
        buf[4] = self.reserved1;
        buf[5] = self.dl as u8;
        buf[6] = (self.dl >> 8) as u8;
        buf[7] = self.timer as u8;
        buf[8] = (self.timer >> 8) as u8;

        buf
    }
}
/// オンデマンド通信におけるサブヘッダ
pub struct SlmpSubHeaderOd {
    /// ネットワーク番号
    pub net_no: u8,
    /// 局番
    pub node_no: u8,
    /// IOユニット番号
    pub dst_proc_no: u16,
    /// 予約もしくはマルチドロップ番号
    pub reserved1: u8,
    /// データ長
    pub dl: u16,
    /// 予約
    pub reserved3: u16,
}
impl SlmpSubHeaderOd {
    pub fn from(buf: [u8; 9]) -> SlmpSubHeaderOd {
        let net_no = buf[0];
        let node_no = buf[1];
        let dst_proc_no = (buf[2] as u16) + ((buf[3] as u16) << 8);
        let reserved1 = buf[4];
        let dl = (buf[5] as u16) + ((buf[6] as u16) << 8);
        let reserved3 = (buf[7] as u16) + ((buf[8] as u16) << 8);
        SlmpSubHeaderOd {
            net_no,
            node_no,
            dst_proc_no,
            reserved1,
            dl,
            reserved3,
        }
    }
}
