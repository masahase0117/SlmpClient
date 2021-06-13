use crate::enums::SLMPCommand;
use crate::packing::{
    pack_bits_by_bit, pack_words_by_word, unpack_bits_by_bit, unpack_words_by_word, SLMPDevice,
    SLMPDeviceBlock, SLMPDeviceBlockData, SLMPDeviceData,
};
use crate::SLMPConnectionInfo;

pub fn send_read_cmd_16(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    target: SLMPDevice,
    count: u16,
    is_bit: bool,
) -> u16 {
    let buf1 = target.pack16();
    let mut buf = [0u8; 6];
    for i in 0..4 {
        buf[i] = buf1[i];
    }
    buf[4] = count as u8;
    buf[5] = (count >> 8) as u8;
    let s_cmd;
    if is_bit {
        s_cmd = 1;
        if buf[4] & 1 == 1 {
            // ビットデータは必ず偶数個
            if buf[4] == 0xff {
                buf[5] += 1;
                buf[4] = 0;
            } else {
                buf[4] += 1;
            }
        }
    } else {
        s_cmd = 0;
    }
    connection_info.send_cmd(timeout, SLMPCommand::DeviceRead, s_cmd, &buf)
}

pub fn send_read_cmd_32(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    target: SLMPDevice,
    count: u16,
    is_bit: bool,
) -> u16 {
    let buf1 = target.pack32();
    let mut buf = [0u8; 8];
    for i in 0..6 {
        buf[i] = buf1[i];
    }
    buf[6] = count as u8;
    buf[7] = (count >> 8) as u8;
    let s_cmd;
    if is_bit {
        s_cmd = 3;
        if buf[4] & 1 == 1 {
            // ビットデータは必ず偶数個
            if buf[4] == 0xff {
                buf[5] += 1;
                buf[4] = 0;
            } else {
                buf[4] += 1;
            }
        }
    } else {
        s_cmd = 2;
    }
    connection_info.send_cmd(timeout, SLMPCommand::DeviceRead, s_cmd, &buf)
}

pub fn decode_read_bit_response(buf: &[u8], target: SLMPDevice) -> Vec<SLMPDeviceData<bool>> {
    let mut ret = Vec::new();
    let mut idx = target.addr;
    let bs = unpack_bits_by_bit(buf);
    for b in bs {
        ret.push(SLMPDeviceData::<bool> {
            dev: SLMPDevice {
                d_code: target.d_code,
                addr: idx,
            },
            value: b,
        });
        idx += 1;
    }
    ret
}

pub fn decode_read_word_response(buf: &[u8], target: SLMPDevice) -> Vec<SLMPDeviceData<u16>> {
    let mut ret = Vec::new();
    let mut idx = target.addr;
    let ws = unpack_words_by_word(buf);
    for w in ws {
        ret.push(SLMPDeviceData::<u16> {
            dev: SLMPDevice {
                d_code: target.d_code,
                addr: idx,
            },
            value: w,
        });
        idx += 1;
    }
    ret
}

pub fn send_write_bit_cmd_16(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    targets: &[SLMPDeviceData<bool>],
) -> u16 {
    let s_cmd = 1;
    let mut buf = Vec::new();
    for d in targets[0].dev.pack16().iter() {
        buf.push(*d);
    }
    buf.push(targets.len() as u8);
    buf.push((targets.len() >> 8) as u8);
    let mut d_buf = Vec::new();
    for d in targets.iter() {
        d_buf.push(d.value);
    }
    let d_buf = pack_bits_by_bit(d_buf.as_slice());
    for d in d_buf.iter() {
        buf.push(*d);
    }
    connection_info.send_cmd(timeout, SLMPCommand::DeviceWrite, s_cmd, buf.as_slice())
}

pub fn send_write_bit_cmd_32(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    targets: &[SLMPDeviceData<bool>],
) -> u16 {
    let s_cmd = 3;
    let mut buf = Vec::new();
    for d in targets[0].dev.pack32().iter() {
        buf.push(*d);
    }
    buf.push(targets.len() as u8);
    buf.push((targets.len() >> 8) as u8);
    let mut d_buf = Vec::new();
    for d in targets.iter() {
        d_buf.push(d.value);
    }
    let d_buf = pack_bits_by_bit(d_buf.as_slice());
    for d in d_buf.iter() {
        buf.push(*d);
    }
    connection_info.send_cmd(timeout, SLMPCommand::DeviceWrite, s_cmd, buf.as_slice())
}

pub fn send_write_word_cmd_16(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    targets: &[SLMPDeviceData<u16>],
) -> u16 {
    let s_cmd = 0;
    let mut buf = Vec::new();
    for d in targets[0].dev.pack16().iter() {
        buf.push(*d);
    }
    buf.push(targets.len() as u8);
    buf.push((targets.len() >> 8) as u8);
    let mut d_buf = Vec::new();
    for d in targets.iter() {
        d_buf.push(d.value);
    }
    let d_buf = pack_words_by_word(d_buf.as_slice());
    for d in d_buf.iter() {
        buf.push(*d);
    }
    connection_info.send_cmd(timeout, SLMPCommand::DeviceWrite, s_cmd, buf.as_slice())
}

pub fn send_write_word_cmd_32(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    targets: &[SLMPDeviceData<u16>],
) -> u16 {
    let s_cmd = 2;
    let mut buf = Vec::new();
    for d in targets[0].dev.pack32().iter() {
        buf.push(*d);
    }
    buf.push(targets.len() as u8);
    buf.push((targets.len() >> 8) as u8);
    let mut d_buf = Vec::new();
    for d in targets.iter() {
        d_buf.push(d.value);
    }
    let d_buf = pack_words_by_word(d_buf.as_slice());
    for d in d_buf.iter() {
        buf.push(*d);
    }
    connection_info.send_cmd(timeout, SLMPCommand::DeviceWrite, s_cmd, buf.as_slice())
}

pub fn send_read_random_cmd_16(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    target_word: &[SLMPDevice],
    target_dword: &[SLMPDevice],
) -> u16 {
    let s_cmd = 0;
    let mut buf = Vec::new();
    if target_word.len() > 0xff {
        panic!("Too many word target");
    }
    if target_dword.len() > 0xff {
        panic!("Too many dword target")
    }
    buf.push(target_word.len() as u8);
    buf.push(target_dword.len() as u8);
    for dev in target_word {
        for d in dev.pack16().iter() {
            buf.push(*d);
        }
    }
    for dev in target_dword {
        for d in dev.pack16().iter() {
            buf.push(*d);
        }
    }
    connection_info.send_cmd(
        timeout,
        SLMPCommand::DeviceReadRandom,
        s_cmd,
        buf.as_slice(),
    )
}

pub fn send_read_random_cmd_32(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    target_word: &[SLMPDevice],
    target_dword: &[SLMPDevice],
) -> u16 {
    let s_cmd = 2;
    let mut buf = Vec::new();
    if target_word.len() > 0xff {
        panic!("Too many word target");
    }
    if target_dword.len() > 0xff {
        panic!("Too many dword target")
    }
    buf.push(target_word.len() as u8);
    buf.push(target_dword.len() as u8);
    for dev in target_word {
        for d in dev.pack32().iter() {
            buf.push(*d);
        }
    }
    for dev in target_dword {
        for d in dev.pack32().iter() {
            buf.push(*d);
        }
    }
    connection_info.send_cmd(
        timeout,
        SLMPCommand::DeviceReadRandom,
        s_cmd,
        buf.as_slice(),
    )
}

pub fn decode_read_random_response(
    buf: &[u8],
    target_word: &[SLMPDevice],
    target_dword: &[SLMPDevice],
) -> (Vec<SLMPDeviceData<u16>>, Vec<SLMPDeviceData<u32>>) {
    let mut ret_word = Vec::new();
    let mut ret_dword = Vec::new();
    let mut tmp_buf = Vec::new();
    tmp_buf.copy_from_slice(buf);
    for t in target_word {
        let d_low = tmp_buf.remove(0) as u16;
        let d_high = tmp_buf.remove(0) as u16;
        ret_word.push(SLMPDeviceData {
            dev: *t,
            value: d_low + d_high << 8,
        });
    }
    for t in target_dword {
        let d_1 = tmp_buf.remove(0) as u32;
        let d_2 = tmp_buf.remove(0) as u32;
        let d_3 = tmp_buf.remove(0) as u32;
        let d_4 = tmp_buf.remove(0) as u32;
        ret_dword.push(SLMPDeviceData {
            dev: *t,
            value: d_1 + (d_2 << 8) + (d_3 << 16) + (d_4 << 24),
        });
    }
    (ret_word, ret_dword)
}

pub fn send_write_random_bits_cmd_16(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    targets: &[SLMPDeviceData<bool>],
) -> u16 {
    let s_cmd = 1;
    let mut buf = Vec::new();
    if targets.len() > 0xff {
        panic!("Too many targets")
    }
    buf.push(targets.len() as u8);
    for dd in targets {
        for d in dd.pack16().iter() {
            buf.push(*d);
        }
        match dd.value {
            true => buf.push(1),
            false => buf.push(0),
        }
    }
    connection_info.send_cmd(
        timeout,
        SLMPCommand::DeviceWriteRandom,
        s_cmd,
        buf.as_slice(),
    )
}
pub fn send_write_random_bits_cmd_32(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    targets: &[SLMPDeviceData<bool>],
) -> u16 {
    let s_cmd = 3;
    let mut buf = Vec::new();
    if targets.len() > 0xff {
        panic!("Too many targets")
    }
    buf.push(targets.len() as u8);
    for dd in targets {
        for d in dd.pack32().iter() {
            buf.push(*d);
        }
        match dd.value {
            true => buf.push(1),
            false => buf.push(0),
        }
        buf.push(0);
    }
    connection_info.send_cmd(
        timeout,
        SLMPCommand::DeviceWriteRandom,
        s_cmd,
        buf.as_slice(),
    )
}
pub fn send_write_random_words_cmd_16(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    target_word: &[SLMPDeviceData<u16>],
    target_dword: &[SLMPDeviceData<u32>],
) -> u16 {
    let s_cmd = 0;
    let mut buf = Vec::new();
    if target_word.len() > 0xff {
        panic!("Too many word target");
    }
    if target_dword.len() > 0xff {
        panic!("Too many dword target")
    }
    buf.push(target_word.len() as u8);
    buf.push(target_dword.len() as u8);
    for dd in target_word {
        for d in dd.pack16().iter() {
            buf.push(*d);
        }
        buf.push(dd.value as u8);
        buf.push((dd.value >> 8) as u8);
    }
    for dd in target_dword {
        for d in dd.pack16().iter() {
            buf.push(*d);
        }
        buf.push(dd.value as u8);
        buf.push((dd.value >> 8) as u8);
        buf.push((dd.value >> 16) as u8);
        buf.push((dd.value >> 24) as u8);
    }

    connection_info.send_cmd(
        timeout,
        SLMPCommand::DeviceWriteRandom,
        s_cmd,
        buf.as_slice(),
    )
}
pub fn send_write_random_words_cmd_32(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    target_word: &[SLMPDeviceData<u16>],
    target_dword: &[SLMPDeviceData<u32>],
) -> u16 {
    let s_cmd = 2;
    let mut buf = Vec::new();
    if target_word.len() > 0xff {
        panic!("Too many word target");
    }
    if target_dword.len() > 0xff {
        panic!("Too many dword target")
    }
    buf.push(target_word.len() as u8);
    buf.push(target_dword.len() as u8);
    for dd in target_word {
        for d in dd.pack32().iter() {
            buf.push(*d);
        }
        buf.push(dd.value as u8);
        buf.push((dd.value >> 8) as u8);
    }
    for dd in target_dword {
        for d in dd.pack32().iter() {
            buf.push(*d);
        }
        buf.push(dd.value as u8);
        buf.push((dd.value >> 8) as u8);
        buf.push((dd.value >> 16) as u8);
        buf.push((dd.value >> 24) as u8);
    }

    connection_info.send_cmd(
        timeout,
        SLMPCommand::DeviceWriteRandom,
        s_cmd,
        buf.as_slice(),
    )
}
pub fn send_entry_monitor_device_cmd_16(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    target_word: &[SLMPDevice],
    target_dword: &[SLMPDevice],
) -> u16 {
    let s_cmd = 0;
    let mut buf = Vec::new();
    if target_word.len() > 0xff {
        panic!("Too many word target");
    }
    if target_dword.len() > 0xff {
        panic!("Too many dword target")
    }
    buf.push(target_word.len() as u8);
    buf.push(target_dword.len() as u8);
    for dev in target_word {
        for d in dev.pack16().iter() {
            buf.push(*d);
        }
    }
    for dev in target_dword {
        for d in dev.pack16().iter() {
            buf.push(*d);
        }
    }
    connection_info.send_cmd(
        timeout,
        SLMPCommand::EntryMonitorDevice,
        s_cmd,
        buf.as_slice(),
    )
}
pub fn send_entry_monitor_device_cmd_32(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    target_word: &[SLMPDevice],
    target_dword: &[SLMPDevice],
) -> u16 {
    let s_cmd = 2;
    let mut buf = Vec::new();
    if target_word.len() > 0xff {
        panic!("Too many word target");
    }
    if target_dword.len() > 0xff {
        panic!("Too many dword target")
    }
    buf.push(target_word.len() as u8);
    buf.push(target_dword.len() as u8);
    for dev in target_word {
        for d in dev.pack32().iter() {
            buf.push(*d);
        }
    }
    for dev in target_dword {
        for d in dev.pack32().iter() {
            buf.push(*d);
        }
    }
    connection_info.send_cmd(
        timeout,
        SLMPCommand::EntryMonitorDevice,
        s_cmd,
        buf.as_slice(),
    )
}
pub fn send_execute_monitor_cmd(connection_info: &mut SLMPConnectionInfo, timeout: u16) -> u16 {
    connection_info.send_cmd(timeout, SLMPCommand::ExecuteMonitor, 0, &[])
}
pub fn send_read_block_cmd_16(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    target_word: &[SLMPDeviceBlock],
    target_bit: &[SLMPDeviceBlock],
) -> u16 {
    let s_cmd = 0;
    let mut buf = Vec::new();
    if target_word.len() > 0xff {
        panic!("Too many word target");
    }
    if target_bit.len() > 0xff {
        panic!("Too many bit target")
    }
    buf.push(target_word.len() as u8);
    buf.push(target_bit.len() as u8);
    for db in target_word.iter() {
        for d in db.pack16().iter() {
            buf.push(*d);
        }
    }
    for db in target_bit.iter() {
        for d in db.pack16().iter() {
            buf.push(*d);
        }
    }

    connection_info.send_cmd(timeout, SLMPCommand::ReadBlock, s_cmd, &buf)
}
pub fn send_read_block_cmd_32(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    target_word: &[SLMPDeviceBlock],
    target_bit: &[SLMPDeviceBlock],
) -> u16 {
    let s_cmd = 2;
    let mut buf = Vec::new();
    if target_word.len() > 0xff {
        panic!("Too many word target");
    }
    if target_bit.len() > 0xff {
        panic!("Too many bit target")
    }
    buf.push(target_word.len() as u8);
    buf.push(target_bit.len() as u8);
    for db in target_word.iter() {
        for d in db.pack32().iter() {
            buf.push(*d);
        }
    }
    for db in target_bit.iter() {
        for d in db.pack32().iter() {
            buf.push(*d);
        }
    }

    connection_info.send_cmd(timeout, SLMPCommand::ReadBlock, s_cmd, &buf)
}
pub fn decode_read_block_response(
    buf: &[u8],
    target_word: &[SLMPDeviceBlock],
    target_bit: &[SLMPDeviceBlock],
) -> (
    Vec<SLMPDeviceBlockData<u16>>,
    Vec<SLMPDeviceBlockData<bool>>,
) {
    let mut ret_w = Vec::new();
    let mut ret_b = Vec::new();
    let mut buf = Vec::from(buf);
    for db in target_word.iter() {
        let mut dbd = SLMPDeviceBlockData::<u16>::new(db.top_device, db.count);
        let mut tmp = Vec::new();
        for _ in 0..db.count * 2 {
            tmp.push(buf.remove(0));
        }
        dbd.decode(&tmp);
        ret_w.push(dbd);
    }
    for db in target_bit.iter() {
        let mut dbd = SLMPDeviceBlockData::<bool>::new(db.top_device, db.count);
        let mut tmp = Vec::new();
        for _ in 0..db.count * 2 {
            tmp.push(buf.remove(0));
        }
        dbd.decode(&tmp);
        ret_b.push(dbd);
    }

    (ret_w, ret_b)
}
pub fn send_write_block_cmd_16(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    target_word: &[SLMPDeviceBlockData<u16>],
    target_bit: &[SLMPDeviceBlockData<bool>],
) -> u16 {
    let s_cmd = 0;
    let mut buf = Vec::new();
    if target_word.len() > 0xff {
        panic!("Too many word target");
    }
    if target_bit.len() > 0xff {
        panic!("Too many bit target")
    }
    buf.push(target_word.len() as u8);
    buf.push(target_bit.len() as u8);
    for dbd in target_word.iter() {
        buf.extend_from_slice(&dbd.pack16());
    }
    for dbd in target_bit.iter() {
        buf.extend_from_slice(&dbd.pack16());
    }
    connection_info.send_cmd(timeout, SLMPCommand::WriteBlock, s_cmd, &buf)
}
pub fn send_write_block_cmd_32(
    connection_info: &mut SLMPConnectionInfo,
    timeout: u16,
    target_word: &[SLMPDeviceBlockData<u16>],
    target_bit: &[SLMPDeviceBlockData<bool>],
) -> u16 {
    let s_cmd = 2;
    let mut buf = Vec::new();
    if target_word.len() > 0xff {
        panic!("Too many word target");
    }
    if target_bit.len() > 0xff {
        panic!("Too many bit target")
    }
    buf.push(target_word.len() as u8);
    buf.push(target_bit.len() as u8);
    for dbd in target_word.iter() {
        buf.extend_from_slice(&dbd.pack32());
    }
    for dbd in target_bit.iter() {
        buf.extend_from_slice(&dbd.pack32());
    }
    connection_info.send_cmd(timeout, SLMPCommand::WriteBlock, s_cmd, &buf)
}
