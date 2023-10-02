use {
    crate::{boxed_error, genesis::DEFAULT_MAX_GENESIS_ARCHIVE_UNPACKED_SIZE, LEDGER_DIR},
    log::*,
    std::{error::Error, process::Command},
};

fn ledger_directory_exists() -> Result<(), Box<dyn Error>> {
    if !LEDGER_DIR.exists() {
        return Err(boxed_error!(format!(
            "Ledger Directory does not exist, have you created genesis yet??"
        )));
    }
    Ok(())
}

fn parse_u16_from_str(input: &str) -> Result<u16, Box<dyn Error>> {
    let result = input.trim().parse::<u16>()?;
    Ok(result)
}

pub struct LedgerHelper {}

impl LedgerHelper {
    pub fn get_shred_version() -> Result<u16, Box<dyn Error>> {
        ledger_directory_exists()?;
        let output = Command::new("solana-ledger-tool")
            .arg("-l")
            .arg(LEDGER_DIR.as_os_str())
            .arg("shred-version")
            .arg("--max-genesis-archive-unpacked-size")
            .arg(DEFAULT_MAX_GENESIS_ARCHIVE_UNPACKED_SIZE.to_string())
            .output()
            .expect("Failed to execute create-snapshot command");

        if output.status.success() {
            let shred_version = parse_u16_from_str(&String::from_utf8_lossy(&output.stdout))?;
            info!("Shred Version: {}", shred_version);
            Ok(shred_version)
        } else {
            Err(boxed_error!(format!(
                "Error in solana-ledger-tool create-snapshot command. err: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }

    pub fn create_snapshot(warp_slot: u64) -> Result<(), Box<dyn Error>> {
        ledger_directory_exists()?;
        let config_dir = LEDGER_DIR.join("accounts_hash_cache");
        if config_dir.exists() {
            std::fs::remove_dir_all(&config_dir).unwrap();
        }
        std::fs::create_dir_all(&config_dir).unwrap();
        let output = Command::new("solana-ledger-tool")
            .arg("-l")
            .arg(LEDGER_DIR.as_os_str())
            .arg("create-snapshot")
            .arg("0")
            .arg(LEDGER_DIR.as_os_str())
            .arg("--warp-slot")
            .arg(warp_slot.to_string())
            .output()
            .expect("Failed to execute create-snapshot command");

        if !output.status.success() {
            return Err(boxed_error!(format!(
                "Error in solana-ledger-tool create-snapshot command. err: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        Ok(())
    }

    pub fn create_bank_hash() -> Result<String, Box<dyn Error>> {
        ledger_directory_exists()?;
        let output = Command::new("solana-ledger-tool")
            .arg("-l")
            .arg(LEDGER_DIR.as_os_str())
            .arg("bank-hash")
            .arg("--halt-at-slot")
            .arg(0.to_string())
            .output()
            .expect("Failed to execute bank-hash command");

        if output.status.success() {
            let bank_hash = String::from_utf8_lossy(&output.stdout);
            Ok(bank_hash.trim().to_string())
        } else {
            Err(boxed_error!(format!(
                "Error in solana-ledger-tool bank-hash command. err: {}",
                String::from_utf8_lossy(&output.stderr)
            )))
        }
    }
}
