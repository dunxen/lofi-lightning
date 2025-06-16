use std::{str::FromStr, sync::Arc, thread, time::Duration};

use ldk_node::{
    bip39::{self, Mnemonic},
    bitcoin::{secp256k1::PublicKey, Address, Network},
    lightning::ln::msgs::SocketAddress,
    lightning_invoice::{Bolt11InvoiceDescription, Description, DEFAULT_EXPIRY_TIME},
    Builder, Node,
};
use rustyline::{error::ReadlineError, DefaultEditor};

fn main() -> anyhow::Result<()> {
    // --- STEP 1: Setup ---
    // Initialize an LDK Node Builder. Allows us to set various parameters for our node
    // before creating it, following the builder pattern, which is common in Rust.
    let mut builder = Builder::new();
    // We will be using Mutinynet, which is a variant of signet with 30 seconds between blocks.
    builder.set_network(Network::Signet);
    // We provide an esplora-compatible chain source for our network. Alternatively, you could
    // provide an electrum or bitcoind_rpc source.
    builder.set_chain_source_esplora("https://mutinynet.com/api/".into(), None);
    // Instead of using P2P to source gossip data, we use Mutinynet's Rapid Gossip Sync
    // snapshot server by LDK.
    builder.set_gossip_source_rgs("https://rgs.mutinynet.com/snapshot".into());
    // If the storage path is not set it defaults to somewhere like /tmp/ldk-node.
    // This contains our keys seed, node data, and logs.
    builder.set_storage_dir_path("./ldk-node".into());

    // If you want to use an existing BIP39 mnemonic as entropy.
    // DON'T JUST DO THIS IN PROD PLS <3
    // let mnemonic = Mnemonic::from_str(include_str!("../bip39seed"))?;
    // builder.set_entropy_bip39_mnemonic(mnemonic, None);

    let node = Arc::new(builder.build()?);

    node.start()?;

    let event_node = node.clone();
    thread::spawn(|| handle_events(event_node));

    println!("Starting our LoFi Lightning Node...");
    println!("Our Node ID: {}", node.node_id());

    let mut rl = DefaultEditor::new()?;

    // We start the loop for reading from stdin
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let words: Vec<&str> = line.split_whitespace().collect();
                parse_and_execute_command(words, &node)?;
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    node.stop()?;
    Ok(())
}

fn parse_and_execute_command(words: Vec<&str>, node: &Node) -> anyhow::Result<()> {
    if words.len() < 1 {
        println!("Invalid command");
        return Ok(());
    }

    match words[0] {
        "listpeers" => {
            let peers = node.list_peers();
            if peers.is_empty() {
                println!("No connected peers");
                return Ok(());
            }
            for peer in peers {
                println!("{peer:?}");
            }
        }
        "listchannels" => {
            let channels = node.list_channels();
            if channels.is_empty() {
                println!("No known channels");
                return Ok(());
            }
            for channel in channels {
                println!("{channel:?}");
            }
        }
        "getnewfundingaddress" => {
            let funding_address = node.onchain_payment().new_address()?;
            println!("New funding address: {funding_address}");
        }
        "openchannel" => {
            if words.len() < 2 {
                println!("Please provide the node ID, network address, and channel amount");
                return Ok(());
            }

            if words.len() < 3 {
                println!("Please provide the network address and channel amount");
                return Ok(());
            }

            if words.len() < 4 {
                println!("Please provide the channel amount");
                return Ok(());
            }

            let node_id = PublicKey::from_str(words[1])?;
            let network_address = SocketAddress::from_str(words[2]).unwrap();
            let amount: u64 = words[3].parse()?;

            let user_channel_id =
                node.open_channel(node_id, network_address, amount, None, None)?;

            println!("User channel ID: {}", user_channel_id.0);
        }
        "sendtoaddress" => {
            if words.len() < 2 {
                println!("Please provide the address and sats amount");
                return Ok(());
            }

            if words.len() < 3 {
                println!("Please provide the sats amount");
                return Ok(());
            }

            let address = Address::from_str(words[1])?.require_network(Network::Signet)?;
            let amount_sats: u64 = words[2].parse()?;

            let onchain = node.onchain_payment();
            let txid = onchain.send_to_address(&address, amount_sats, None)?;
            println!("Sending {amount_sats} sats to {address} onchain via {txid}");
        }
        "listbalances" => {
            let balances = node.list_balances();
            println!(
                "Total onchain balance: {} sats\nSpendable onchain balance {} sats\nTotal lightning balance {} sats",
                balances.total_onchain_balance_sats,
                balances.spendable_onchain_balance_sats,
                balances.total_lightning_balance_sats
            );
        }
        "paykeysend" => {
            if words.len() < 2 {
                println!("Please provide the msat amount");
                return Ok(());
            }

            if words.len() < 3 {
                println!("Please provide the node id");
                return Ok(());
            }

            let amount_msat: u64 = words[1].parse()?;
            let node_id = PublicKey::from_str(words[2])?;

            let payment = node.spontaneous_payment();
            payment.send(amount_msat, node_id, None)?;
        }
        "createinvoice" => {
            let description = Bolt11InvoiceDescription::Direct(Description::new(
                words.get(2).unwrap_or(&"Lightning invoice").to_string(),
            )?);
            let invoice =
                if let Some(Ok(amount_msat)) = words.get(1).map(|value| value.parse::<u64>()) {
                    node.bolt11_payment().receive(
                        amount_msat,
                        &description,
                        DEFAULT_EXPIRY_TIME as u32,
                    )?
                } else {
                    node.bolt11_payment()
                        .receive_variable_amount(&description, DEFAULT_EXPIRY_TIME as u32)?
                };
            println!("{invoice}");
        }
        _ => {
            println!("Invalid command");
            return Ok(());
        }
    }

    Ok(())
}

fn handle_events(node: Arc<Node>) -> anyhow::Result<()> {
    loop {
        if let Some(event) = node.next_event() {
            println!("{event:?}");
            node.event_handled()?;
        } else {
            thread::sleep(Duration::from_secs(1));
        }
    }
}
