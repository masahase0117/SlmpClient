/// SLMPで定義されているコマンド
#[derive(Copy, Clone, Debug, Hash)]
pub enum SLMPCommand {
    // Device
    DeviceRead = 0x0401,
    DeviceWrite = 0x1401,
    DeviceReadRandom = 0x0403,
    DeviceWriteRandom = 0x1402,
    EntryMonitorDevice = 0x0801,
    ExecuteMonitor = 0x0802,
    ReadBlock = 0x0406,
    WriteBlock = 0x01406,
    // Label
    ArrayLabelRead = 0x041A,
    ArrayLabelWrite = 0x141A,
    LabelReadRandom = 0x041C,
    LabelWriteRandom = 0x141B,
    // Memory
    MemoryRead = 0x0613,
    MemoryWrite = 0x1613,
    // ExtendUnit
    ExtendUnitRead = 0x0601,
    ExtendUnitWrite = 0x1601,
    // RemoteControl
    RemoteRun = 0x1001,
    RemoteStop = 0x1002,
    RemotePause = 0x1003,
    RemoteLatchClear = 0x1005,
    RemoteReset = 0x1006,
    ReadTypeName = 0x0101,
    NodeIndication = 0x3070,
    // Drive
    ReadDiskState = 0x0205,
    Defrag = 0x1207,
    // RemotePassword
    RemotePasswordLock = 0x1631,
    RemotePasswordUnlock = 0x1630,
    // File
    ReadFileInfo = 0x0201,
    ReadFileInfoWithTitle = 0x0202,
    ReadFileNoInfo = 0x0204,
    ChangeFileInfo = 0x1204,
    FileSearch = 0x0203,
    FileRead = 0x0206,
    FileWrite = 0x1203,
    FileLock = 0x0808,
    FileCopy = 0x1206,
    FileDelete = 0x1205,
    ReadDir = 0x1810,
    SearchDir = 0x1811,
    NewFileA = 0x1202,
    NewFileB = 0x1820,
    DeleteFile = 0x1822,
    CopyFile = 0x1824,
    ChangeFileState = 0x1825,
    ChangeFileDate = 0x1826,
    OpenFile = 0x1827,
    ReadFile = 0x1828,
    WriteFile = 0x1829,
    CloseFile = 0x182A,
    SelfTest = 0x0619,
    ClearErrorCode = 0x1617,
    ClearErrorHistory = 0x1619,
    OnDemand = 0x2101,
    // DataCollection
    Auth = 0x4000,
    KeepAlive = 0x4001,
    GetData = 0x4002,
    Distribute = 0x4003,
    // NodeConnection
    NodeSearch = 0x0E30,
    IPAddressSet = 0x0E31,
    // ParameterSetting
    DeviceInfoCompare = 0x0E32,
    ParameterGet = 0x0E33,
    ParameterUpdate = 0x0E34,
    ParameterSetStart = 0x0E35,
    ParameterSetEnd = 0x0E36,
    ParameterSetCancel = 0x0E3A,
    DeviceIdentificationInfoGet = 0x0E28,
    CommunicationSpeed = 0x3072,
    // NodeMonitoring
    StatusRead = 0x0E44,
    StatusRead2 = 0x0E53,
    ConnectionSettingGet = 0x0E45,
    DataMonitoring = 0x0E29,
    // Other
    CAN = 0x4020,
    IOLInk = 0x5000,
    ModbusTCP = 0x5001,
    ModbusRTU = 0x5002,
    // CCLinkIEFieldDiagnostics
    SelectNodeInfoGet = 0x3119,
    CommunicationTest = 0x3040,
    CableTest = 0x3050,
    // CCLinkIETSNNetworkManagement
    NetworkConfig = 0x0E90,
    MasterConfig = 0x0E91,
    SlaveConfig = 0x0E92,
    CyclicConfig = 0x0E93,
    Notification = 0x0E94,
    // LinkDeviceParameter
    LinkDevicePrmWrite = 0x320A,
    LinkDevicePrmWriteCheckReq = 0x320B,
    LinkDevicePrmWriteCheckResp = 0x320C,
    // EventHistory
    GetEventNum = 0x3060,
    GetEventHistory = 0x3061,
    ClearEventHistory = 0x161A,
    ClockOffsetDataSend = 0x3062,
    // BackupRestore
    GetCommunicationSet = 0x0EB0,
    GetStationSubIDList = 0x0EB1,
    GetDeviceInfo = 0x0EB2,
    StartBackup = 0x0EB3,
    EndBackup = 0x0EB4,
    RequestBackup = 0x0EB5,
    GetBackupPrm = 0x0EB6,
    CheckRestore = 0x0EB7,
    StartRestore = 0x0EB8,
    EndRestore = 0x0EB9,
    SetBackupPrm = 0x0EBA,
    // SlaveStationPrmRestore_
    CheckPrmDelivery = 0x0EBE,
    // StartStopCyclic
    StopOwnStationCyclic = 0x3206,
    StartOwnStationCyclic = 0x3207,
    StopOtherStationCyclic = 0x3208,
    StartOtherStationCyclic = 0x3209,
    //ReservedStation
    RsvStationConfigTemporaryRelease = 0x320D,
    RsvStationConfig = 0x320E,
    // WatchdogCounter
    SetWatchdogCounterInfo = 0x3210,
    WatchdogCounterOffsetConfig = 0x3211,
}

/// SLMPで定義されているデバイス
#[derive(Copy, Clone, Debug, Hash)]
pub enum SLMPDeviceCode {
    SM = 0x91,
    SD = 0xA9,
    X = 0x9C,
    Y = 0x9D,
    M = 0x90,
    L = 0x92,
    F = 0x93,
    V = 0x94,
    B = 0xA0,
    D = 0xA8,
    W = 0xB4,
    TS = 0xC1,
    TC = 0xC0,
    TN = 0xC2,
    LTS = 0x51,
    LTC = 0x50,
    LTN = 0x52,
    STS = 0xC7,
    STC = 0xC6,
    STN = 0xC8,
    LSTS = 0x59,
    LSTC = 0x58,
    LSTN = 0x5A,
    CS = 0xC4,
    CC = 0xC3,
    CN = 0xC5,
    SB = 0xA1,
    SW = 0xB5,
    DX = 0xA2,
    DY = 0xA3,
    Z = 0xCC,
    LZ = 0x62,
    R = 0xAF,
    ZR = 0xB0,
    RD = 0x2C,
    LCS = 0x55,
    LCC = 0x54,
    LCN = 0x56,
}

/// SLMPで定義されている終了コード
#[derive(Copy, Clone, Debug, Hash)]
pub enum SLMPEndCode {
    Success = 0x00,
    WrongCommand = 0xC059,
    WrongFormat = 0xC05C,
    WrongLength = 0xC061,
    Busy = 0xCEE0,
    ExceedReqLength = 0xCEE1,
    ExceedRespLength = 0xCEE2,
    ServerNotFound = 0xCF10,
    WrongConfigItem = 0xCF20,
    PrmIDNotFound = 0xCF30,
    NotStartExclusiveWrite = 0xCF31,
    RelayFailure = 0xCF70,
    TimeoutError = 0xCF71,
    CANAppNotPermittedRead = 0xCCC7,
    CANAppWriteOnly = 0xCCC8,
    CANAppReadOnly = 0xCCC9,
    CANAppUndefinedObjectAccess = 0xCCCA,
    CANAppNotPermittedPDOMapping = 0xCCCB,
    CANAppExceedPDOMapping = 0xCCCC,
    CANAppNotExistSubIndex = 0xCCD3,
    CANAppWrongParameter = 0xCCD4,
    CANAppMoreOverParameterRange = 0xCCD5,
    CANAppLessOverParameterRange = 0xCCD6,
    CANAppTransOrStoreError = 0xCCDA,
    CANAppOtherError = 0xCCFF,
    OtherNetworkError = 0xCF00,
    DataFragmentShortage = 0xCF40,
    DataFragmentDup = 0xCF41,
    DataFragmentLost = 0xCF43,
    DataFragmentNotSupport = 0xCF44,
}

extern crate num;
use num::traits::FromPrimitive;
use std::option::Option;

impl SLMPEndCode {
    /// 終了コード番号から対応するコードを得る
    pub fn get(code: u16) -> Option<SLMPEndCode> {
        match code {
            0x00 => Some(SLMPEndCode::Success),
            0xC059 => Some(SLMPEndCode::WrongCommand),
            0xC05C => Some(SLMPEndCode::WrongFormat),
            0xC061 => Some(SLMPEndCode::WrongLength),
            0xCEE0 => Some(SLMPEndCode::Busy),
            0xCEE1 => Some(SLMPEndCode::ExceedReqLength),
            0xCEE2 => Some(SLMPEndCode::ExceedRespLength),
            0xCF10 => Some(SLMPEndCode::ServerNotFound),
            0xCF20 => Some(SLMPEndCode::WrongConfigItem),
            0xCF30 => Some(SLMPEndCode::PrmIDNotFound),
            0xCF31 => Some(SLMPEndCode::NotStartExclusiveWrite),
            0xCF70 => Some(SLMPEndCode::RelayFailure),
            0xCF71 => Some(SLMPEndCode::TimeoutError),
            0xCCC7 => Some(SLMPEndCode::CANAppNotPermittedRead),
            0xCCC8 => Some(SLMPEndCode::CANAppWriteOnly),
            0xCCC9 => Some(SLMPEndCode::CANAppReadOnly),
            0xCCCA => Some(SLMPEndCode::CANAppUndefinedObjectAccess),
            0xCCCB => Some(SLMPEndCode::CANAppNotPermittedPDOMapping),
            0xCCCC => Some(SLMPEndCode::CANAppExceedPDOMapping),
            0xCCD3 => Some(SLMPEndCode::CANAppNotExistSubIndex),
            0xCCD4 => Some(SLMPEndCode::CANAppWrongParameter),
            0xCCD5 => Some(SLMPEndCode::CANAppMoreOverParameterRange),
            0xCCD6 => Some(SLMPEndCode::CANAppLessOverParameterRange),
            0xCCDA => Some(SLMPEndCode::CANAppTransOrStoreError),
            0xCCFF => Some(SLMPEndCode::CANAppOtherError),
            0xCF00 => Some(SLMPEndCode::OtherNetworkError),
            0xCF40 => Some(SLMPEndCode::DataFragmentShortage),
            0xCF41 => Some(SLMPEndCode::DataFragmentDup),
            0xCF43 => Some(SLMPEndCode::DataFragmentLost),
            0xCF44 => Some(SLMPEndCode::DataFragmentNotSupport),
            _ => None,
        }
    }
}

impl FromPrimitive for SLMPEndCode {
    fn from_i64(n: i64) -> Option<SLMPEndCode> {
        SLMPEndCode::get(n as u16)
    }
    fn from_u16(n: u16) -> Option<SLMPEndCode> {
        SLMPEndCode::get(n)
    }
    fn from_u32(n: u32) -> Option<SLMPEndCode> {
        SLMPEndCode::get(n as u16)
    }
    fn from_u64(n: u64) -> Option<SLMPEndCode> {
        SLMPEndCode::get(n as u16)
    }
}

impl PartialEq for SLMPCommand {
    fn eq(&self, other: &Self) -> bool {
        (*self as u32) == (*other as u32)
    }
}
impl PartialEq for SLMPDeviceCode {
    fn eq(&self, other: &Self) -> bool {
        (*self as u32) == (*other as u32)
    }
}
impl PartialEq for SLMPEndCode {
    fn eq(&self, other: &Self) -> bool {
        (*self as u32) == (*other as u32)
    }
}

#[derive(Copy, Clone, Debug, Hash)]
pub enum SLMPClearMode {
    Not = 0x00,
    Clear = 0x01,
    AllClear = 0x02,
}

use std::error::Error;
use std::io;
use std::net::{SocketAddr, TcpStream, UdpSocket};
use std::time::Duration;

/// TCPとUDPの差異を吸収
pub enum TCPorUDP {
    UDP(UdpSocket, SocketAddr),
    TCP(TcpStream),
}
impl TCPorUDP {
    /// 指定されたバイト列を送信する
    /// # 引数
    ///
    /// * `buf` - 送信するバイト列
    ///
    /// # 返値
    ///
    /// 送信したバイト数
    ///
    pub fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        use std::io::Write;
        use TCPorUDP::*;
        match self {
            TCP(stream) => stream.write(buf),
            UDP(socket, remote) => socket.send_to(buf, remote.to_string()),
        }
    }
    /// 受信する
    /// # 引数
    ///
    /// * `buf` - 受信したデータを入れるバッファ、十分な量を確保しておくこと
    ///
    /// # 返値
    ///
    /// 受信したバイト数
    pub fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        use TCPorUDP::*;
        match self {
            TCP(stream) => stream.peek(buf),
            UDP(socket, _) => socket.recv(buf),
        }
    }
    /// 読み取りタイムアウトの設定
    /// # 引数
    ///
    /// * `timeout` - ミリ秒単位での指定
    ///
    pub fn set_timeout(&mut self, timeout: u64) {
        use TCPorUDP::*;
        let result = match self {
            TCP(stream) => stream.set_read_timeout(Some(Duration::from_millis(timeout))),
            UDP(socket, _) => socket.set_read_timeout(Some(Duration::from_millis(timeout))),
        };
        match result {
            Ok(_) => (),
            Err(e) => eprintln!("{}", e),
        }
    }
}
