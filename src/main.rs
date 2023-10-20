//! And here? Does the package doc go to main?

use atp_id::{did::plc::DidPlc, handle::Handle};
use clap::Parser;
use miette::Result;

mod cmdlargs;

use cmdlargs::{Cli, Command};

#[tokio::main]
async fn main() -> Result<()> {
    // Get command line arguments.
    let args = Cli::parse();

    match &args.command {
        // Simple lookup operations
        Command::Lookup { id } => {
            if !id.contains("did:plc") {
                match Handle::try_from(id.as_str()) {
                    Ok(handle) => match DidPlc::from_handle(handle).await {
                        Ok(did) => println!("{did:?}"),
                        Err(err) => {
                            dbg!(err);
                        }
                    },
                    Err(err) => {
                        dbg!(err);
                    }
                }
            } else {
                match Handle::from_did(id).await {
                    Ok(maybe) => {
                        if let Some(handles) = maybe {
                            println!("Handles from DID Document:");
                            handles.iter().for_each(|handle| {
                                println!("{handle}");
                            })
                        } else {
                            println!("DID Document was empty")
                        }
                    }
                    Err(err) => eprintln!("{err}"),
                }
            }
        }
        // The real interesting stuff
        Command::Query(_query_arguments) => eprintln!("Move along, nothing to see here..."),
    }

    Ok(())
}
