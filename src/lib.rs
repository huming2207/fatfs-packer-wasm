mod utils;

use std::{io::Cursor, ops::Range, path::Path, collections::btree_map::Entry};

use fatfs::{FileSystem, FsOptions, format_volume, FormatVolumeOptions};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct FatFsPacker {
    buf: Box<[u8]>,
    sector_size: u16,
    sector_count: u32,
}

#[wasm_bindgen]
impl FatFsPacker {
    #[wasm_bindgen(constructor)]
    pub fn new(sector_size: u16, sector_count: u32) -> FatFsPacker {
        let buf: Box<[u8]> = vec![0; (sector_count * (sector_size as u32)) as usize].into_boxed_slice();
        FatFsPacker { buf, sector_size, sector_count }
    }

    #[wasm_bindgen]
    pub fn format(self) -> bool {
        let options = FormatVolumeOptions::new().total_sectors(self.sector_count).bytes_per_sector(self.sector_size);
        match format_volume(Cursor::new(self.buf), options) {
            Ok(()) => true,
            Err(_) => false,
        }
    }

    #[wasm_bindgen]
    pub fn add_data(self, buf: &[u8], name: &str) -> bool {
        let fs = match FileSystem::new(Cursor::new(self.buf), FsOptions::new()) {
            Ok(fs) => fs,
            Err(_) => return false,
        };
    
        let root = fs.root_dir();
        let path = Path::new(name);
        for entry in path.read_dir().expect("Can't parse directory") {
            if let Ok(entry) = entry {
                
            }
        }


        false
    }
}