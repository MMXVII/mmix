mod_def_reexport!{
    bdifi,
    bdif,
    mori,
    mor,
    mxori,
    mxor,
    odifi,
    odif,
    tdifi,
    tdif,
    wdifi,
    wdif,
}

fn get_byte(bits: u64) -> u8 {
    bits as u8
}

fn get_wyde(bits: u64) -> u16 {
    bits as u16
}

fn get_tetra(bits: u64) -> u32 {
    bits as u32
}
