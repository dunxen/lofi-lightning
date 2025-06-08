use std::{sync::Arc, thread};

use ldk_node::{bitcoin::Network, Builder, Node};
use rustyline::DefaultEditor;

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
        // --- STEP 2: Use rustyline to read from stdin
        //             Start by handling CTRL-C and CTRL-D keyboard interrupts which should stop
        //             the lightning node gracefully. These appear as ReadlineError::Interrupted
        //             and RealineError::Eof in the Err() case respectively.
        //             If a line is successfully read (the Ok() case), then it should be parsed
        //             as a command and run if possible by calling out to parse_and_execute_command.
        //
        // TODO
    }
}

fn parse_and_execute_command(words: Vec<&str>, node: &Node) -> anyhow::Result<()> {
    // TODO: Implement me by parsing commands and their arguments and then call the appropriate
    //       methods on Node.
    //
    // Some commands to implement:
    //     listpeers: list the peers connected to this node
    //     listchannels: list all channels of this node
    //     getnewfundingaddress: get a new onchain address to fund this node's onchain wallet
    //     openchannel: open a channel to a given peer with a given amount of msats
    //     sendtoaddress: send a given amount of sats onchain to an address
    //     listbalances: list all balances of this node including onchain and offchain balances
    //     paykeysend: pay over lightning directly to a node ID using spontaneous payments (keysend)
    //     createinvoice: create a single use BOLT 11 invoice
    //     ... and more!
    Ok(())
}

fn handle_events(node: Arc<Node>) -> anyhow::Result<()> {
    loop {
        // TODO: Print the next event if available and mark it as handled so ldk-node doesn't keep
        //       emitting it, else put the thread to sleep for a bit.
    }
}
