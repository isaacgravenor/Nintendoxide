pub struct PRegisters {
    //Map to memory somewhere?
    pub ppuctrl: u8,    //$2000, write only
    pub ppumask: u8,    //$2001, write only
    pub ppustatus: u8,  //$2002 etc. - read only
    pub oamaddr: u8,    //write only
    pub oamdata: u8,    //read/write
    pub ppuscroll: u16, //2 byte state read by two 1 byte accesses
    pub ppuaddr: u16,   //same as above
    pub ppudata: u8,    //read/write
    pub oamdma: u8,     //$4014, write
}
