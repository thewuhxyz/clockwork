mod client;
use client::*;
use clockwork_thread_program::state::BigInstruction;

use {
    anchor_lang::{
        prelude::*,
        solana_program::{instruction::Instruction, system_program},
        InstructionData, ToAccountMetas,
    },
    anyhow::Result,
    bincode::serialize,
    clockwork_thread_program::state::{LookupTables, Thread, Trigger},
    serde_json::json,
    solana_address_lookup_table_program::state::AddressLookupTable,
    solana_client::{rpc_config::RpcSendTransactionConfig, rpc_request::RpcRequest},
    solana_sdk::{
        address_lookup_table_account::AddressLookupTableAccount,
        commitment_config::{CommitmentConfig, CommitmentLevel},
        message::{v0, VersionedMessage},
        native_token::LAMPORTS_PER_SOL,
        signature::{read_keypair_file, Signature},
        signers::Signers,
        transaction::{Transaction, VersionedTransaction},
    },
    solana_transaction_status::UiTransactionEncoding,
    std::{str::FromStr, thread, time},
};

fn main() -> Result<()> {
    // Creating a Client with your default paper keypair as payer
    let client = default_client();
    let app_localnet_simul_pk =
        Pubkey::from_str("GuJVu6wky7zeVaPkGaasC5vx1eVoiySbEv7UFKZAu837").unwrap();
    client.airdrop(&app_localnet_simul_pk, LAMPORTS_PER_SOL)?;

    println!("Create the address lookup table");
    let recent_slot = client
        .get_slot_with_commitment(CommitmentConfig::finalized())
        .unwrap();
    let lut_auth = client.payer_pubkey();
    let (create_ix, lut) = solana_address_lookup_table_program::instruction::create_lookup_table(
        lut_auth,
        client.payer_pubkey(),
        recent_slot,
    );
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    client
        .send_and_confirm_transaction(&Transaction::new_signed_with_payer(
            &[create_ix],
            Some(&client.payer_pubkey()),
            &[client.payer()],
            latest_blockhash,
        ))
        .unwrap();

    println!("Create all stores");
    let mut keys: Vec<Pubkey> = Vec::new();
    for i in 0..60 {
        let store_key =
            Pubkey::find_program_address(&[format!("{i}").as_bytes()], &lookup_tables::ID).0;

        let create_store_ix = Instruction {
            accounts: lookup_tables::accounts::CreateStore {
                payer: client.payer_pubkey(),
                system_program: system_program::ID,
                store: store_key,
            }
            .to_account_metas(None),
            program_id: lookup_tables::ID,
            data: lookup_tables::instruction::CreateStore {
                seed: format!("{i}"),
            }
            .data(),
        };

        match client.get_account(&store_key) {
            Ok(_) => keys.push(store_key),
            Err(_) => {
                client.send_and_confirm_transaction(&Transaction::new_signed_with_payer(
                    &[create_store_ix],
                    Some(&client.payer_pubkey()),
                    &[&client.payer],
                    latest_blockhash,
                ))?;
                keys.push(store_key)
            }
        }

        println!("Created store {i}")
    }

    println!("Loop to extend the address lookup table");
    let mut signature = Signature::default();
    let latest_blockhash = client.get_latest_blockhash().unwrap();
    for keys in keys.chunks(20) {
        let extend_ix = solana_address_lookup_table_program::instruction::extend_lookup_table(
            lut,
            lut_auth,
            Some(client.payer_pubkey()),
            keys.into(),
        );

        signature = client
            .send_and_confirm_transaction(&Transaction::new_signed_with_payer(
                &[extend_ix],
                Some(&client.payer_pubkey()),
                &[&client.payer],
                latest_blockhash,
            ))
            .unwrap();
    }
    client
        .confirm_transaction_with_spinner(
            &signature,
            &latest_blockhash,
            CommitmentConfig::finalized(),
        )
        .unwrap();

    println!("Create Big Instruction");
    let tsi = chrono::Local::now();
    let big_ix_id = format!("{}_{}", "big ix id", tsi.format("%d_%H:%M:%S"));

    let big_ix_key = BigInstruction::pubkey(
        client.payer_pubkey(),
        lookup_tables::ID,
        big_ix_id.try_to_vec()?,
    );

    println!("Big instruction key: {:#?}", big_ix_key);

    thread::sleep(time::Duration::from_secs(5));

    // instruction to add to thread
    let add_to_store_ix = Instruction {
        accounts: lookup_tables::accounts::AddToStore {
            store_0: keys[0],
            store_1: keys[1],
            store_2: keys[2],
            store_3: keys[3],
            store_4: keys[4],
            store_5: keys[5],
            store_6: keys[6],
            store_7: keys[7],
            store_8: keys[8],
            store_9: keys[9],
            store_10: keys[10],
            store_11: keys[11],
            store_12: keys[12],
            store_13: keys[13],
            store_14: keys[14],
            store_15: keys[15],
            store_16: keys[16],
            store_17: keys[17],
            store_18: keys[18],
            store_19: keys[19],
            store_20: keys[20],
            store_21: keys[21],
            store_22: keys[22],
            store_23: keys[23],
            store_24: keys[24],
            store_25: keys[25],
            store_26: keys[26],
            store_27: keys[27],
            store_28: keys[28],
            store_29: keys[29],
            store_30: keys[30],
            store_31: keys[31],
            store_32: keys[32],
            store_33: keys[33],
            store_34: keys[34],
            store_35: keys[35],
            store_36: keys[36],
            store_37: keys[37],
            store_38: keys[38],
            store_39: keys[39],
            store_40: keys[40],
            store_41: keys[41],
            store_42: keys[42],
            store_43: keys[43],
            store_44: keys[44],
            store_45: keys[45],
            store_46: keys[46],
            store_47: keys[47],
            store_48: keys[48],
            store_49: keys[49],
            store_50: keys[50],
            store_51: keys[51],
            store_52: keys[52],
            store_53: keys[53],
            store_54: keys[54],
            store_55: keys[55],
            store_56: keys[56],
            store_57: keys[57],
            store_58: keys[58],
            store_59: keys[59],
        }
        .to_account_metas(Some(false)),
        program_id: lookup_tables::ID,
        data: lookup_tables::instruction::AddToStore { data: 1u8 }.data(),
    };

    // get accounts from the instruction
    let accounts_from_add_to_store_ix = add_to_store_ix.clone().accounts;

    // accounts to create big instruction
    let mut accounts_from_big_ix_create =
        clockwork_thread_program::accounts::BigInstructionCreate {
            authority: client.payer_pubkey(),
            payer: client.payer_pubkey(),
            system_program: system_program::ID,
            instruction_program_id: add_to_store_ix.clone().program_id,
            big_instruction: big_ix_key,
        }
        .to_account_metas(None);

    // append the accounts needed for the instrucion with the accounts to create big instruction. it must be in the same order
    accounts_from_big_ix_create.extend(accounts_from_add_to_store_ix);

    // instruction to create big instruction with account for the instruction to add to thread appended
    let create_big_ix = Instruction {
        accounts: accounts_from_big_ix_create,
        program_id: clockwork_thread_program::ID,
        data: clockwork_thread_program::instruction::BigInstructionCreate {
            id: big_ix_id.try_to_vec()?,
            instruction_data: add_to_store_ix.clone().data,
            no_of_accounts: add_to_store_ix.accounts.len() as u8,
        }
        .data(),
    };

    // send transaction
    let versioned_tx =
        create_tx_with_address_table_lookup(&client, &[create_big_ix], lut, &[&client.payer])?;
    let serialized_versioned_tx = serialize(&versioned_tx)?;
    println!(
        "The serialized versioned tx is {} bytes",
        serialized_versioned_tx.len()
    );
    let serialized_encoded = base64::encode(serialized_versioned_tx);
    let config = RpcSendTransactionConfig {
        skip_preflight: true,
        preflight_commitment: Some(CommitmentLevel::Processed),
        encoding: Some(UiTransactionEncoding::Base64),
        ..RpcSendTransactionConfig::default()
    };

    let signature = client
        .send::<String>(
            RpcRequest::SendTransaction,
            json!([serialized_encoded, config]),
        )
        .unwrap();

    println!("create big ix: https://explorer.solana.com/tx/{}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899", signature);
    client
        .confirm_transaction_with_commitment(
            &Signature::from_str(&signature).unwrap(),
            CommitmentConfig::finalized(),
        )
        .unwrap();

    // Thread stuff
    let ts = chrono::Local::now();
    let thread_id = format!("{}_{}", "lutrs", ts.format("%d_%H:%M:%S"));
    let thread_auth = client.payer_pubkey();
    let thread = Thread::pubkey(thread_auth, thread_id.clone().into());

    // Dummy instruction as we cannot pass big instruction when creating thread
    let dummy_ix = Instruction {
        accounts: clockwork_thread_program::accounts::ThreadDummyIx { thread }
            .to_account_metas(None),
        program_id: clockwork_thread_program::ID,
        data: clockwork_thread_program::instruction::ThreadDummyIx {}.data(),
    };

    // thread create ix
    let thread_create_ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: clockwork_thread_program::accounts::ThreadCreate {
            authority: client.payer_pubkey(),
            payer: client.payer_pubkey(),
            system_program: system_program::ID,
            thread,
        }
        .to_account_metas(Some(false)),
        data: clockwork_thread_program::instruction::ThreadCreate {
            amount: LAMPORTS_PER_SOL,
            id: thread_id.into(),
            instructions: vec![dummy_ix.clone().into()],
            trigger: Trigger::Cron {
                schedule: "*/10 * * * * * *".into(),
                skippable: true,
            },
        }
        .data(),
    };

    // Add LookupTables to Thread
    let thread_lut = LookupTables::pubkey(thread_auth, thread);
    let create_thread_lut_ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: clockwork_thread_program::accounts::LookupTablesCreate {
            authority: client.payer_pubkey(),
            payer: client.payer_pubkey(),
            system_program: system_program::ID,
            thread,
            lookup_tables: thread_lut,
        }
        .to_account_metas(Some(false)),
        data: clockwork_thread_program::instruction::ThreadLookupTablesCreate {
            address_lookup_tables: vec![lut],
        }
        .data(),
    };

    // Add big instruction to thread
    let thread_add_big_ix = Instruction {
        program_id: clockwork_thread_program::ID,
        accounts: clockwork_thread_program::accounts::ThreadBigInstructionAdd {
            authority: client.payer_pubkey(),
            system_program: system_program::ID,
            thread,
            big_instruction: big_ix_key,
        }
        .to_account_metas(Some(false)),
        data: clockwork_thread_program::instruction::ThreadBigInstructionAdd {}.data(),
    };
    println!("thread {:#?}", thread);

    let ixs = [thread_create_ix, create_thread_lut_ix];
    let ixs_2 = [thread_add_big_ix];
    client.send_and_confirm(&ixs, &[&client.payer]).unwrap();
    let sig = client.send_and_confirm(&ixs_2, &[&client.payer]).unwrap();
    println!("add big ix to thread tx: https://explorer.solana.com/tx/{}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899", sig);

    print!("Waiting for threads to execute...");
    // some time for thread to settle and start executing transactions
    thread::sleep(time::Duration::from_secs(30));

    // inspect each signature in the explorer
    let thread_latest_sig = client.get_signatures_for_address(&thread)?[0]
        .signature
        .clone();
    println!("Inspect thread latest sig: https://explorer.solana.com/tx/{thread_latest_sig}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899");
    Ok(())
}

fn create_tx_with_address_table_lookup<T: Signers>(
    client: &Client,
    instructions: &[Instruction],
    address_lookup_table_key: Pubkey,
    signers: &T,
) -> Result<VersionedTransaction> {
    let raw_account = client.get_account(&address_lookup_table_key)?;
    let address_lookup_table = AddressLookupTable::deserialize(&raw_account.data)?;
    let address_lookup_table_account = AddressLookupTableAccount {
        key: address_lookup_table_key,
        addresses: address_lookup_table.addresses.to_vec(),
    };

    let blockhash = client.get_latest_blockhash()?;
    let tx = VersionedTransaction::try_new(
        VersionedMessage::V0(v0::Message::try_compile(
            &client.payer_pubkey(),
            instructions,
            &[address_lookup_table_account],
            blockhash,
        )?),
        signers,
    )?;

    Ok(tx)
}

fn default_client() -> Client {
    let config_file = solana_cli_config::CONFIG_FILE.as_ref().unwrap().as_str();
    let config = solana_cli_config::Config::load(config_file).unwrap();
    let keypair = read_keypair_file(&config.keypair_path).unwrap();
    Client::new(keypair, config.json_rpc_url)
}
