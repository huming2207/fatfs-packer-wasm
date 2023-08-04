mod utils;

use std::{io::{Cursor, Write}, ops::Range, path::{PathBuf, Component}, str::FromStr};

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
    pub fn format(self) -> Result<(), JsError> {
        let options = FormatVolumeOptions::new().total_sectors(self.sector_count).bytes_per_sector(self.sector_size);
        match format_volume(Cursor::new(self.buf), options) {
            Ok(()) => Ok(()),
            Err(err) => Err(JsError::new(format!("{:?}", err).as_str())),
        }
    }

    #[wasm_bindgen(js_name = createPath)]
    pub fn create_path(self, path: &str) -> Result<(), JsError> {
        let fs = match FileSystem::new(Cursor::new(self.buf), FsOptions::new()) {
            Ok(fs) => fs,
            Err(err) => return Err(JsError::new(format!("{:?}", err).as_str())),
        };

        let mut dir = fs.root_dir();
        let path = match PathBuf::from_str(path) {
            Ok(p) => p,
            Err(err) => return Err(JsError::new(format!("{:?}", err).as_str())),
        };

        let path_iter = path.components();
        for entry in path_iter {
            if entry == Component::RootDir {
                continue;
            }

            let entry_str = match entry.as_os_str().to_str() {
                Some(s) => s,
                None => continue,
            };

            dir = match dir.open_dir(entry_str) {
                Ok(d) => {
                    // Enter this directory if exists
                    d
                },
                Err(_) => {
                    // Create directory here if not exists 
                    match dir.create_dir(entry_str) {
                        Ok(d) => d,

                        // If you can't even create, something else must be wrong!
                        Err(err) => return Err(JsError::new(format!("{:?}", err).as_str())),
                    }
                }
            }
        }

        Ok(())
    }

    #[wasm_bindgen(js_name = addData)]
    pub fn add_data(self, buf: &[u8], path: &str) -> Result<(), JsError> {
        let fs = match FileSystem::new(Cursor::new(self.buf), FsOptions::new()) {
            Ok(fs) => fs,
            Err(err) => return Err(JsError::new(format!("{:?}", err).as_str())),
        };
    
        let dir = fs.root_dir();
        let mut file = match dir.create_file(path) {
            Ok(f) => f,
            Err(err) => return Err(JsError::new(format!("{:?}", err).as_str())),
        };

        match file.write_all(buf) {
            Ok(()) => Ok(()),
            Err(err) => return Err(JsError::new(format!("{:?}", err).as_str())),
        }
    }

    #[wasm_bindgen(js_name = generateImage)]
    pub fn generate_image(&self) -> Vec<u8> {
        return self.buf.to_vec();
    }
}