use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use memmem::{Searcher, TwoWaySearcher};
use num::FromPrimitive;
use zip::ZipArchive;

use super::super::{GBAError, GBAResult};
use super::backup::eeprom::*;
use super::backup::flash::*;
use super::backup::{BackupFile, BackupType};
use super::header;
use super::BackupMedia;
use super::Cartridge;

use crate::util::read_bin_file;

#[derive(Debug)]
pub struct GamepakBuilder {
    path: Option<PathBuf>,
    bytes: Option<Box<[u8]>>,
    save_type: BackupType,
    create_backup_file: bool,
}

impl GamepakBuilder {
    pub fn new() -> GamepakBuilder {
        GamepakBuilder {
            save_type: BackupType::AutoDetect,
            path: None,
            bytes: None,
            create_backup_file: true,
        }
    }

    pub fn buffer(mut self, bytes: &[u8]) -> Self {
        self.bytes = Some(bytes.into());
        self
    }

    pub fn file(mut self, path: &Path) -> Self {
        self.path = Some(path.to_path_buf());
        self
    }

    pub fn save_type(mut self, save_type: BackupType) -> Self {
        self.save_type = save_type;
        self
    }

    pub fn with_sram(mut self) -> Self {
        self.save_type = BackupType::Sram;
        self
    }

    pub fn with_flash128k(mut self) -> Self {
        self.save_type = BackupType::Flash1M;
        self
    }

    pub fn with_flash64k(mut self) -> Self {
        self.save_type = BackupType::Flash512;
        self
    }

    pub fn with_eeprom(mut self) -> Self {
        self.save_type = BackupType::Eeprom;
        self
    }

    pub fn without_backup_to_file(mut self) -> Self {
        self.create_backup_file = false;
        self
    }

    pub fn build(mut self) -> GBAResult<Cartridge> {
        let bytes = if let Some(bytes) = self.bytes {
            Ok(bytes)
        } else if let Some(path) = &self.path {
            let loaded_rom = load_rom(&path)?;
            Ok(loaded_rom.into())
        } else {
            Err(GBAError::CartridgeLoadError(
                "either provide file() or buffer()".to_string(),
            ))
        }?;

        let header = header::parse(&bytes);
        info!("Loaded ROM: {:?}", header);

        if !self.create_backup_file {
            self.path = None;
        }

        if self.save_type == BackupType::AutoDetect {
            if let Some(detected) = detect_backup_type(&bytes) {
                info!("Detected Backup: {:?}", detected);
                self.save_type = detected;
            } else {
                warn!("could not detect backup save type");
            }
        }

        let backup = create_backup(self.save_type, self.path);

        let size = bytes.len();
        Ok(Cartridge {
            header: header,
            bytes: bytes,
            size: size,
            backup: backup,
        })
    }
}

const BACKUP_FILE_EXT: &'static str = "sav";
fn create_backup(backup_type: BackupType, rom_path: Option<PathBuf>) -> BackupMedia {
    let backup_path = if let Some(rom_path) = rom_path {
        Some(rom_path.with_extension(BACKUP_FILE_EXT))
    } else {
        None
    };
    match backup_type {
        BackupType::Flash | BackupType::Flash512 => {
            BackupMedia::Flash(Flash::new(backup_path, FlashSize::Flash64k))
        }
        BackupType::Flash1M => BackupMedia::Flash(Flash::new(backup_path, FlashSize::Flash128k)),
        BackupType::Sram => BackupMedia::Sram(BackupFile::new(0x8000, backup_path)),
        BackupType::Eeprom => BackupMedia::Eeprom(EepromController::new(backup_path)),
        BackupType::AutoDetect => BackupMedia::Undetected,
    }
}

fn detect_backup_type(bytes: &[u8]) -> Option<BackupType> {
    const ID_STRINGS: &'static [&'static str] =
        &["EEPROM", "SRAM", "FLASH_", "FLASH512_", "FLASH1M_"];

    for i in 0..5 {
        let search = TwoWaySearcher::new(ID_STRINGS[i].as_bytes());
        match search.search_in(bytes) {
            Some(_) => return Some(BackupType::from_u8(i as u8).unwrap()),
            _ => {}
        }
    }
    None
}

fn load_rom(path: &Path) -> GBAResult<Vec<u8>> {
    match path.extension() {
        Some(extension) => match extension.to_str() {
            Some("zip") => {
                let zipfile = File::open(path)?;
                let mut archive = ZipArchive::new(zipfile)?;
                for i in 0..archive.len() {
                    let mut file = archive.by_index(i)?;
                    if file.name().ends_with(".gba") {
                        let mut buf = Vec::new();
                        file.read_to_end(&mut buf)?;
                        return Ok(buf);
                    }
                }
                panic!("no .gba file contained in the zip file");
            }
            _ => {
                let buf = read_bin_file(path)?;
                return Ok(buf);
            }
        },
        _ => {
            let buf = read_bin_file(path)?;
            return Ok(buf);
        }
    }
}
