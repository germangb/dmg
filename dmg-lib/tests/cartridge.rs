static ROM: &[u8] = include_bytes!("roms/10-print.gb");

use dmg_lib::{cartridge::Rom, map::Mapped, mmu::Mmu, ppu::palette::GRAYSCALE, Mode};

#[test]
fn checksum() {
    let mmu = Mmu::with_cartridge_and_video(Rom::new(ROM), Mode::GB, ());

    let mut res = 0x19u8;
    for addr in 0x134..=0x14d {
        res = res.wrapping_add(mmu.read(addr as u16));
    }

    assert_eq!(0, res);
}

#[test]
#[ignore]
fn rom_only() {
    unimplemented!()
}

#[test]
#[ignore]
fn mbc1() {
    unimplemented!()
}

#[test]
#[ignore]
fn mbc3() {
    unimplemented!()
}
