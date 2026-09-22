struct PRegisters {
    //Map to memory somewhere?
    ppuctrl: u8,    //$2000, write only
    ppumask: u8,    //$2001, write only
    ppustatus: u8,  //$2002 etc. - read only
    oamaddr: u8,    //write only
    oamdata: u8,    //read/write
    ppuscroll: u16, //2 byte state read by two 1 byte accesses
    ppuaddr: u16,   //same as above
    ppudata: u8,    //read/write
    oamdma: u8,     //$4014, write
}
