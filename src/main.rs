fn main() {
    //scalar
    const INT1:i8=-1_28; //-128~127
    const INT2:u8=255; //0~255
    const INT3:isize=0x00; //based on system architecture
    
    
    println!("{},{},{}", INT1, INT2, INT3);
}
