/**
 * File containing structures for reading and parsing
 * TIFF files
 **/

 /// Represents a TIF header
 struct TIFHEAD {
     /// Byte-order Identifier
     Identifier: u16,
     /// TIFF Version number (always 2Ah)
     Version: u16,
     /// Offset of first Image File Directory
     IDFOffset: u32
 }

 struct TIFIFD {
     NumDirEntries: u16,
     Taglist: Vec<u8>,
     NextIFDOffset: u32
 }

