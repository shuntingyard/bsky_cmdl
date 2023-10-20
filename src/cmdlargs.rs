use clap::{Args, Parser, Subcommand};
use url::Url;

/// Doc comment
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
// #[command(CMD ATTRIBUTE)]
// #[group(GROUP ATTRIBUTE)]
pub struct Cli {
    /*
    /// Doc comment
    #[arg(ARG ATTRIBUTE)]
    field: UserType,

    #[arg(value_enum, ARG ATTRIBUTE...)]
    field: EnumValues,

    #[command(flatten)]
    delegate: Struct,
     */
    #[command(subcommand)]
    pub command: Command,
}

/*
/// Doc comment
#[derive(Args)]
#[command(PARENT CMD ATTRIBUTE)]
#[group(GROUP ATTRIBUTE)]
struct Struct {
    /// Doc comment
    #[command(ARG ATTRIBUTE)]
    field: UserType,
}
 */

/// Doc comment
#[derive(Subcommand)]
//#[command(PARENT CMD ATTRIBUTE)]
pub enum Command {
    /// Doc comment
    // #[command(query)]
    Query(QueryArguments),

    /// Handle and DID PLC, unauthenticated lookups
    // #[command(lookup)]
    Lookup {
        /// Enter some user.host.tld or some did:plc:j4fgugpzzggivsd7c3hkksvv
        id: String,
    },
}

#[derive(Args)]
pub struct QueryArguments {
    /// AT Protocol service to connect to
    #[clap(short = 's', long = "service", env)]
    atp_service: Url,
    /// Username to log in with
    #[clap(short = 'u', long = "user", env)]
    atp_username: String,
    /// Password to log in with
    #[clap(short = 'p', long = "password", env)]
    atp_password: String,
}

/*
/// Doc comment
#[derive(ValueEnum)]
// #[value(VALUE ENUM ATTRIBUTE)]
enum EnumValues {
    /// Doc comment
    #[value(POSSIBLE VALUE ATTRIBUTE)]
    Variant1,
}
 */
