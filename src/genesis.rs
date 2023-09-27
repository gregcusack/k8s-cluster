//! A command-line executable for generating the chain's genesis config.
#![allow(clippy::arithmetic_side_effects)]

use std::process::Output;

use {
    crate::{boxed_error, initialize_globals, SOLANA_ROOT},
    log::*,
    std::{
        error::Error,
        path::PathBuf,
        process::Command,
    },
};

// pub const DEFAULT_WORD_COUNT: usize = 12;
pub const DEFAULT_FAUCET_LAMPORTS: u64 = 500000000000000000;
pub const DEFAULT_MAX_GENESIS_ARCHIVE_UNPACKED_SIZE: u64 = 1073741824;
// pub const DEFAULT_CLUSTER_TYPE: ClusterType = ClusterType::Development;
// pub const DEFAULT_COMMISSION: u8 = 100;
pub const DEFAULT_INTERNAL_NODE_STAKE_SOL: f64 = 10.0; // 10000000000 lamports
pub const DEFAULT_INTERNAL_NODE_SOL: f64 = 500.0; // 500000000000 lamports
pub const DEFAULT_BOOTSTRAP_NODE_STAKE_LAMPORTS: u64 = 1000000000; // 1 SOL
pub const DEFAULT_BOOTSTRAP_NODE_LAMPORTS: u64 = 500000000000; // 500 SOL

// fn output_keypair(keypair: &Keypair, outfile: &str, source: &str) -> Result<(), Box<dyn Error>> {
//     if outfile == STDOUT_OUTFILE_TOKEN {
//         let mut stdout = std::io::stdout();
//         write_keypair(keypair, &mut stdout)?;
//     } else {
//         write_keypair_file(keypair, outfile)?;
//         println!("Wrote {source} keypair to {outfile}");
//     }
//     Ok(())
// }

// fn generate_keypair() -> Result<Keypair, Box<dyn Error>> {
//     let (passphrase, _) = keygen::mnemonic::no_passphrase_and_message();
//     let mnemonic_type = MnemonicType::for_word_count(DEFAULT_WORD_COUNT).unwrap();
//     let mnemonic = Mnemonic::new(mnemonic_type, Language::English);
//     let seed = Seed::new(&mnemonic, &passphrase);
//     keypair_from_seed(seed.as_bytes())
// }

// fn fetch_spl(fetch_spl_file: &PathBuf) -> Result<(), Box<dyn Error>> {
//     let output = Command::new("bash")
//         .arg(fetch_spl_file)
//         .output() // Capture the output of the script
//         .expect("Failed to run fetch-spl.sh script");

//     // Check if the script execution was successful
//     if output.status.success() {
//         Ok(())
//     } else {
//         Err(boxed_error!(format!(
//             "Failed to fun fetch-spl.sh script: {}",
//             String::from_utf8_lossy(&output.stderr)
//         )))
//     }
// }

// fn parse_spl_genesis_file(
//     spl_file: &PathBuf,
// ) -> Result<HashMap<String, Vec<SPLGenesisArgType>>, Box<dyn Error>> {
//     if let Ok(file) = File::open(spl_file) {
//         let reader = BufReader::new(file);

//         // Read the first line of the file
//         if let Some(line) = reader.lines().next() {
//             let line_contents = line?;

//             // Split the line into individual parts using "--" as the delimiter
//             let parts: Vec<&str> = line_contents.split("--").collect();

//             // Initialize a HashMap to store parsed arguments
//             let mut parsed_args = HashMap::new();

//             for part in &parts[1..] {
//                 // Trim leading and trailing whitespaces
//                 let trimmed_part = part.trim();

//                 // Split each part into flag and value using the first space
//                 let split_parts: Vec<&str> = trimmed_part.splitn(2, ' ').collect();

//                 if split_parts.len() == 2 {
//                     let flag = format!("--{}", split_parts[0].trim());
//                     // Split the "value" into three parts using spaces
//                     let value_parts: Vec<&str> = split_parts[1].split_whitespace().collect();
//                     if flag == "--bpf-program" && value_parts.len() == 3 {
//                         // Handle "--bpf-program" with 3 arguments
//                         let value_tuple = SPLGenesisArgType::BpfProgram(
//                             value_parts[0].to_string(),
//                             value_parts[1].to_string(),
//                             value_parts[2].to_string(),
//                         );

//                         // Add the flag and value tuple to the parsed_args HashMap
//                         parsed_args
//                             .entry(flag)
//                             .or_insert(Vec::new())
//                             .push(value_tuple);
//                     } else if flag == "--upgradeable-program" && value_parts.len() == 4 {
//                         // Handle "--upgradable-program" with 4 arguments
//                         let value_tuple = SPLGenesisArgType::UpgradeableProgram(
//                             value_parts[0].to_string(),
//                             value_parts[1].to_string(),
//                             value_parts[2].to_string(),
//                             value_parts[3].to_string(),
//                         );

//                         // Add the flag and value tuple to the parsed_args HashMap
//                         parsed_args
//                             .entry(flag)
//                             .or_insert(Vec::new())
//                             .push(value_tuple);
//                     } else {
//                         panic!(
//                             "Invalid argument passed in! flag: {}, value_parts: {}",
//                             flag,
//                             value_parts.len()
//                         );
//                     }
//                 }
//             }
//             return Ok(parsed_args);
//         }
//     }
//     Err(boxed_error!("Can't open spl file even though it exists"))
// }

// enum SPLGenesisArgType {
//     BpfProgram(String, String, String),
//     UpgradeableProgram(String, String, String, String),
// }

pub struct GenesisFlags {
    pub hashes_per_tick: String,
    pub slots_per_epoch: Option<u64>,
    pub target_lamports_per_signature: Option<u64>,
    pub faucet_lamports: Option<u64>,
    pub enable_warmup_epochs: bool,
    pub max_genesis_archive_unpacked_size: Option<u64>,
    pub cluster_type: String,
    pub bootstrap_validator_lamports: Option<f64>,
    pub bootstrap_validator_stake_lamports: Option<f64>,
}

impl std::fmt::Display for GenesisFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "GenesisFlags {{\n\
             hashes_per_tick: {:?},\n\
             slots_per_epoch: {:?},\n\
             target_lamports_per_signature: {:?},\n\
             faucet_lamports: {:?},\n\
             enable_warmup_epochs: {},\n\
             max_genesis_archive_unpacked_size: {:?},\n\
             bootstrap_validator_sol: {:?},\n\
             bootstrap_validator_stake_sol: {:?},\n\
             }}",
            self.hashes_per_tick,
            self.slots_per_epoch,
            self.target_lamports_per_signature,
            self.faucet_lamports,
            self.enable_warmup_epochs,
            self.max_genesis_archive_unpacked_size,
            self.bootstrap_validator_lamports,
            self.bootstrap_validator_stake_lamports,
        )
    }
}

#[derive(Clone, Debug)]
pub struct SetupConfig<'a> {
    pub namespace: &'a str,
    pub num_validators: i32,
    pub prebuild_genesis: bool,
}

pub struct Genesis {
    pub flags: GenesisFlags,
    pub config_dir: PathBuf,
    pub args: Vec<String>,
}

impl Genesis {
    pub fn new(flags: GenesisFlags) -> Self {
        initialize_globals();
        let config_dir = SOLANA_ROOT.join("config");
        if config_dir.exists() {
            std::fs::remove_dir_all(&config_dir).unwrap();
        }
        std::fs::create_dir_all(&config_dir).unwrap();
        Genesis {
            flags,
            config_dir,
            args: Vec::default(),
        }
    }

    pub fn generate_keypair(
        &self, output_path: PathBuf
    ) -> Output {
        Command::new("solana-keygen")
            .arg("new")
            .arg("--no-bip39-passphrase")
            .arg("--silent")
            .arg("-o")
            .arg(output_path)
            .output() // Capture the output of the script
            .expect("Failed to execute solana-keygen for new keypair")
    }

    pub fn generate_faucet(&mut self) -> Result<(), Box<dyn Error>> {
        let outfile = self.config_dir.join("faucet.json");
        let output = self.generate_keypair(outfile);
        if !output.status.success() {
            return Err(boxed_error!(format!(
                "Failed to create new faucet keypair. err: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        Ok(())
    }

    pub fn generate_accounts(
        &mut self,
        validator_type: &str,
        number_of_accounts: i32,
    ) -> Result<(), Box<dyn Error>> {
        let mut filename_prefix = "validator".to_string();
        if validator_type == "bootstrap" {
            filename_prefix = format!("{}-{}", validator_type, filename_prefix);
        } else if validator_type == "validator" {
            filename_prefix = "validator".to_string();
        } else {
            return Err(boxed_error!(format!(
                "Invalid validator type: {}",
                validator_type
            )));
        }

        for i in 0..number_of_accounts {
            self.generate_account(validator_type, filename_prefix.as_str(), i)?;
        }

        Ok(())
    }

    // Create identity, stake, and vote accounts
    fn generate_account(
        &mut self,
        validator_type: &str,
        filename_prefix: &str,
        i: i32,
    ) -> Result<(), Box<dyn Error>> {
        let account_types = vec!["identity", "vote-account", "stake-account"];

        for account in account_types {
            let filename: String;
            if validator_type == "bootstrap" {
                filename = format!("{}/{}.json", filename_prefix, account);
            } else if validator_type == "validator" {
                filename = format!("{}-{}-{}.json", filename_prefix, account, i);
            } else {
                return Err(boxed_error!(format!(
                    "Invalid validator type: {}",
                    validator_type
                )));
            }

            let outfile = self.config_dir.join(filename);
            trace!("outfile: {:?}", outfile);

            let output = self.generate_keypair(outfile);
            if !output.status.success() {
                return Err(boxed_error!(format!(
                    "Failed to create new account keypair. account: {}, err: {}",
                    account,
                    String::from_utf8_lossy(&output.stderr)
                )));
            }
        }
        Ok(())

    }

    fn setup_genesis_flags(&self) -> Vec<String> {
        let mut args: Vec<String> = Vec::new();
        args.push("--bootstrap-validator-stake-lamports".to_string());
        match self.flags.bootstrap_validator_stake_lamports {
            Some(lamports) => args.push(lamports.to_string()),
            None => args.push(DEFAULT_BOOTSTRAP_NODE_STAKE_LAMPORTS.to_string()),   
        }
        args.push("--bootstrap-validator-lamports".to_string());
        match self.flags.bootstrap_validator_lamports {
            Some(lamports) => args.push(lamports.to_string()),
            None => args.push(DEFAULT_BOOTSTRAP_NODE_LAMPORTS.to_string()),   
        }

        args.extend(vec!["--hashes-per-tick".to_string(), self.flags.hashes_per_tick.clone()]);

        args.push("--max-genesis-archive-unpacked-size".to_string());
        match self.flags.max_genesis_archive_unpacked_size {
            Some(size) => args.push(size.to_string()),
            None => args.push(DEFAULT_MAX_GENESIS_ARCHIVE_UNPACKED_SIZE.to_string()),
        }

        if self.flags.enable_warmup_epochs {
            args.push("--enable-warmup-epochs".to_string());
        }

        args.push("--faucet-lamports".to_string());
        match self.flags.faucet_lamports {
            Some(lamports) => args.push(lamports.to_string()),
            None => args.push(DEFAULT_FAUCET_LAMPORTS.to_string()),
        }

        args.extend(vec!["--faucet-pubkey".to_string(), self.config_dir.join("faucet.json").to_string_lossy().to_string()]);
        args.extend(vec!["--cluster-type".to_string(), self.flags.cluster_type.clone()]);
        args.extend(vec!["--ledger".to_string(), self.config_dir.join("bootstrap-validator").to_string_lossy().to_string()]);

        args.extend(
            vec![
                "--bootstrap-validator".to_string(),
                self.config_dir.join("bootstrap-validator/identity.json").to_string_lossy().to_string(),
                self.config_dir.join("bootstrap-validator/stake-account.json").to_string_lossy().to_string(),
                self.config_dir.join("bootstrap-validator/vote-account.json").to_string_lossy().to_string(),
            ]
        );

        if let Some(slots_per_epoch) = self.flags.slots_per_epoch {
            args.extend(vec!["--slots-per-epoch".to_string(), slots_per_epoch.to_string()]);
        }

        if let Some(lamports_per_signature) = self.flags.target_lamports_per_signature {
            args.extend(vec!["--target-lamports-per-signature".to_string(), lamports_per_signature.to_string()]);
        }

        args

        //TODO see multinode-demo.sh. we need spl-genesis-args.sh
        



    }

    pub fn generate(&mut self) -> Result<(), Box<dyn Error>> {
        let output = Command::new("solana-genesis")
            .args(&self.setup_genesis_flags())
            .output() // Capture the output of the script
            .expect("Failed to execute solana-genesis");

        if !output.status.success() {
            return Err(boxed_error!(format!(
                "Failed to create genesis. err: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        Ok(())
    }



    // // Create Genesis
    // pub fn generate(&mut self) -> Result<(), Box<dyn Error>> {
    //     let ledger_path = self.config_dir.join("bootstrap-validator");
    //     let rent = Rent::default();

    //     let bootstrap_validator_lamports = match self.flags.bootstrap_validator_sol {
    //         Some(sol) => sol_to_lamports(sol),
    //         None => sol_to_lamports(DEFAULT_BOOTSTRAP_NODE_SOL)
    //             .max(VoteState::get_rent_exempt_reserve(&rent)),
    //     };

    //     let bootstrap_validator_stake_lamports = match self.flags.bootstrap_validator_stake_sol {
    //         Some(sol) => sol_to_lamports(sol),
    //         None => sol_to_lamports(DEFAULT_BOOTSTRAP_NODE_STAKE_SOL)
    //             .max(rent.minimum_balance(StakeStateV2::size_of())),
    //     };

    //     let faucet_lamports = match self.flags.faucet_lamports {
    //         Some(faucet_lamports) => faucet_lamports,
    //         None => DEFAULT_FAUCET_LAMPORTS,
    //     };

    //     let mut poh_config = PohConfig {
    //         target_tick_duration: Duration::from_micros(timing::duration_as_us(
    //             &PohConfig::default().target_tick_duration,
    //         )),
    //         ..PohConfig::default()
    //     };

    //     let cluster_type = match self.flags.cluster_type {
    //         Some(cluster_type) => cluster_type,
    //         None => DEFAULT_CLUSTER_TYPE,
    //     };

    //     match self.flags.hashes_per_tick {
    //         Some(hashes_per_tick) => poh_config.hashes_per_tick = Some(hashes_per_tick),
    //         None => {
    //             match cluster_type {
    //                 ClusterType::Development => {
    //                     let hashes_per_tick =
    //                         compute_hashes_per_tick(poh_config.target_tick_duration, 1_000_000);
    //                     poh_config.hashes_per_tick = Some(hashes_per_tick / 2); // use 50% of peak ability
    //                 }
    //                 ClusterType::Devnet | ClusterType::Testnet | ClusterType::MainnetBeta => {
    //                     poh_config.hashes_per_tick = Some(clock::DEFAULT_HASHES_PER_TICK);
    //                 }
    //             }
    //         }
    //     }

    //     let max_genesis_archive_unpacked_size = match self.flags.max_genesis_archive_unpacked_size {
    //         Some(max_genesis_archive_unpacked_size) => max_genesis_archive_unpacked_size,
    //         None => DEFAULT_MAX_GENESIS_ARCHIVE_UNPACKED_SIZE,
    //     };

    //     let slots_per_epoch = match self.flags.slots_per_epoch {
    //         Some(slots_per_epoch) => slots_per_epoch,
    //         None => match cluster_type {
    //             ClusterType::Development => clock::DEFAULT_DEV_SLOTS_PER_EPOCH,
    //             ClusterType::Devnet | ClusterType::Testnet | ClusterType::MainnetBeta => {
    //                 clock::DEFAULT_SLOTS_PER_EPOCH
    //             }
    //         },
    //     };

    //     let fee_rate_governor = match self.flags.target_lamports_per_signature {
    //         Some(target_lamports_per_signature) => {
    //             let mut fee_rate_gov = FeeRateGovernor::new(
    //                 target_lamports_per_signature,
    //                 DEFAULT_TARGET_SIGNATURES_PER_SLOT,
    //             );
    //             fee_rate_gov.burn_percent = DEFAULT_BURN_PERCENT;
    //             fee_rate_gov
    //         }
    //         None => FeeRateGovernor::default(),
    //     };

    //     let epoch_schedule = EpochSchedule::custom(
    //         slots_per_epoch,
    //         slots_per_epoch,
    //         self.flags.enable_warmup_epochs,
    //     );

    //     let mut genesis_config = GenesisConfig {
    //         epoch_schedule,
    //         fee_rate_governor,
    //         poh_config,
    //         ..GenesisConfig::default()
    //     };

    //     if let Some(faucet_keypair) = &self.faucet_keypair {
    //         genesis_config.add_account(
    //             faucet_keypair.pubkey(),
    //             AccountSharedData::new(faucet_lamports, 0, &system_program::id()),
    //         );
    //     }

    //     solana_stake_program::add_genesis_accounts(&mut genesis_config);
    //     if genesis_config.cluster_type == ClusterType::Development {
    //         solana_runtime::genesis_utils::activate_all_features(&mut genesis_config);
    //     }

    //     // Based off of genesis/src/main.rs
    //     // loop through this. we only excpect to loop through once for bootstrap.
    //     // but when we add in other validators, we should add these other validator accounts to genesis here
    //     for account in self.validator_keypairs.iter() {
    //         genesis_config.add_account(
    //             account.identity.pubkey(),
    //             AccountSharedData::new(bootstrap_validator_lamports, 0, &system_program::id()),
    //         );

    //         let vote_account = vote_state::create_account_with_authorized(
    //             &account.identity.pubkey(),
    //             &account.identity.pubkey(),
    //             &account.identity.pubkey(),
    //             DEFAULT_COMMISSION,
    //             VoteState::get_rent_exempt_reserve(&rent).max(1),
    //         );

    //         genesis_config.add_account(
    //             account.stake_account.pubkey(),
    //             stake_state::create_account(
    //                 &account.identity.pubkey(),
    //                 &account.vote_account.pubkey(),
    //                 &vote_account,
    //                 &rent,
    //                 bootstrap_validator_stake_lamports,
    //             ),
    //         );

    //         genesis_config.add_account(account.vote_account.pubkey(), vote_account);
    //     }

    //     let issued_lamports = genesis_config
    //         .accounts
    //         .values()
    //         .map(|account| account.lamports)
    //         .sum::<u64>();

    //     add_genesis_accounts(&mut genesis_config, issued_lamports - faucet_lamports);

    //     info!("genesis_config: {}", genesis_config);

    //     let parse_address = |address: &str, input_type: &str| {
    //         address.parse::<Pubkey>().unwrap_or_else(|err| {
    //             eprintln!("Error: invalid {input_type} {address}: {err}");
    //             process::exit(1);
    //         })
    //     };

    //     let parse_program_data = |program: &str| {
    //         let mut program_data = vec![];
    //         File::open(program)
    //             .and_then(|mut file| file.read_to_end(&mut program_data))
    //             .unwrap_or_else(|err| {
    //                 eprintln!("Error: failed to read {program}: {err}");
    //                 process::exit(1);
    //             });
    //         program_data
    //     };

    //     let fetch_spl_file = SOLANA_ROOT.join("fetch-spl.sh");
    //     fetch_spl(&fetch_spl_file)?;

    //     // add in spl stuff
    //     let spl_file = SOLANA_ROOT.join("spl-genesis-args.sh");
    //     // Check if the file exists before reading it
    //     if std::fs::metadata(&spl_file).is_ok() {
    //         let parsed_args = match parse_spl_genesis_file(&spl_file) {
    //             Ok(args) => args,
    //             Err(err) => return Err(err),
    //         };

    //         // HashMap where the keys are flags and the values are vectors of values.
    //         // You can access them as needed.
    //         if let Some(values) = parsed_args.get("--bpf-program") {
    //             for value in values {
    //                 match value {
    //                     SPLGenesisArgType::BpfProgram(address, loader, program) => {
    //                         debug!(
    //                             "Flag: --bpf-program, Address: {}, Loader: {}, Program: {}",
    //                             address, loader, program
    //                         );
    //                         let address = parse_address(address, "address");
    //                         let loader = parse_address(loader, "loader");
    //                         let program_data = parse_program_data(program);
    //                         genesis_config.add_account(
    //                             address,
    //                             AccountSharedData::from(Account {
    //                                 lamports: genesis_config
    //                                     .rent
    //                                     .minimum_balance(program_data.len()),
    //                                 data: program_data,
    //                                 executable: true,
    //                                 owner: loader,
    //                                 rent_epoch: 0,
    //                             }),
    //                         );
    //                     }
    //                     _ => panic!("Incorrect number of values passed in for --bpf-program"),
    //                 }
    //             }
    //         }
    //         if let Some(values) = parsed_args.get("upgradeable-program") {
    //             for value in values {
    //                 match value {
    //                     SPLGenesisArgType::UpgradeableProgram(
    //                         address,
    //                         loader,
    //                         program,
    //                         upgrade_authority,
    //                     ) => {
    //                         debug!(
    //                             "Flag: --upgradeable-program, Address: {}, Loader: {}, Program: {}, upgrade_authority: {}",
    //                             address, loader, program, upgrade_authority
    //                         );
    //                         let address = parse_address(address, "address");
    //                         let loader = parse_address(loader, "loader");
    //                         let program_data_elf = parse_program_data(program);
    //                         let upgrade_authority_address = if upgrade_authority == "none" {
    //                             Pubkey::default()
    //                         } else {
    //                             upgrade_authority.parse::<Pubkey>().unwrap_or_else(|_| {
    //                                 read_keypair_file(upgrade_authority)
    //                                     .map(|keypair| keypair.pubkey())
    //                                     .unwrap_or_else(|err| {
    //                                         eprintln!(
    //                                             "Error: invalid upgrade_authority {upgrade_authority}: {err}"
    //                                         );
    //                                         process::exit(1);
    //                                     })
    //                             })
    //                         };

    //                         let (programdata_address, _) =
    //                             Pubkey::find_program_address(&[address.as_ref()], &loader);
    //                         let mut program_data =
    //                             bincode::serialize(&UpgradeableLoaderState::ProgramData {
    //                                 slot: 0,
    //                                 upgrade_authority_address: Some(upgrade_authority_address),
    //                             })
    //                             .unwrap();
    //                         program_data.extend_from_slice(&program_data_elf);
    //                         genesis_config.add_account(
    //                             programdata_address,
    //                             AccountSharedData::from(Account {
    //                                 lamports: genesis_config
    //                                     .rent
    //                                     .minimum_balance(program_data.len()),
    //                                 data: program_data,
    //                                 owner: loader,
    //                                 executable: false,
    //                                 rent_epoch: 0,
    //                             }),
    //                         );

    //                         let program_data =
    //                             bincode::serialize(&UpgradeableLoaderState::Program {
    //                                 programdata_address,
    //                             })
    //                             .unwrap();
    //                         genesis_config.add_account(
    //                             address,
    //                             AccountSharedData::from(Account {
    //                                 lamports: genesis_config
    //                                     .rent
    //                                     .minimum_balance(program_data.len()),
    //                                 data: program_data,
    //                                 owner: loader,
    //                                 executable: true,
    //                                 rent_epoch: 0,
    //                             }),
    //                         );
    //                     }
    //                     _ => {
    //                         panic!("Incorrect number of values passed in for --upgradeable-program")
    //                     }
    //                 }
    //             }
    //         }
    //     }

    //     create_new_ledger(
    //         &ledger_path,
    //         &genesis_config,
    //         max_genesis_archive_unpacked_size,
    //         LedgerColumnOptions::default(),
    //     )?;

    //     self.genesis_config = Some(genesis_config);

    //     Ok(())
    // }

    // pub fn load_genesis_to_base64_from_file(&self) -> Result<String, Box<dyn Error>> {
    //     let path = self
    //         .config_dir
    //         .join("bootstrap-validator")
    //         .join(DEFAULT_GENESIS_FILE);
    //     let mut input_content = Vec::new();
    //     File::open(path)?.read_to_end(&mut input_content)?;
    //     Ok(general_purpose::STANDARD.encode(input_content))
    // }


    // pub fn ensure_no_dup_pubkeys(&self) {
    //     // Ensure there are no duplicated pubkeys
    //     info!("len of pubkeys: {}", self.all_pubkeys.len());
    //     let mut v = self.all_pubkeys.clone();
    //     v.sort();
    //     v.dedup();
    //     if v.len() != self.all_pubkeys.len() {
    //         panic!("Error: validator pubkeys have duplicates!");
    //     }
    // }
}