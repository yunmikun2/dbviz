mod drawer;
mod loader;
mod schema;

use crate::drawer::dot::Dot;
use crate::drawer::Drawer;
use crate::loader::{postgresql, Loader};

use anyhow::Result;
use structopt::clap::arg_enum;
use structopt::StructOpt;

use std::io::stdout;

arg_enum! {
#[derive(Debug)]
enum LoaderType {
    PostgreSQL,
}}

arg_enum! {
#[derive(Debug)]
enum DrawerType {
    Dot,
}}

#[derive(Debug, StructOpt)]
#[structopt(version = "1.0", author = "yunmikun <yunmikun2@protonmail.com>")]
struct Opts {
    #[structopt(long, default_value = "postgresql")]
    loader: LoaderType,
    #[structopt(long, default_value = "dot")]
    drawer: DrawerType,
    #[structopt(flatten)]
    pg_opts: PgOpts,
}

#[derive(Debug, StructOpt)]
#[structopt(version = "1.0", author = "yunmikun <yunmikun2@protonmail.com>")]
struct PgOpts {
    #[structopt(
        short,
        long,
        default_value = "localhost",
        required_if("pg_opts", "postgresql")
    )]
    hostname: String,
    #[structopt(
        short,
        long,
        default_value = "postgres",
        required_if("pg_opts", "postgresql")
    )]
    username: String,
    #[structopt(
        short,
        long,
        default_value = "postgres",
        required_if("pg_opts", "postgresql")
    )]
    password: String,
    #[structopt(short, long, required_if("pg_opts", "postgresql"))]
    database: Option<String>,
    #[structopt(
        short,
        long,
        default_value = "public",
        required_if("pg_opts", "postgresql")
    )]
    schema: String,
}

fn main() -> Result<()> {
    let opts = Opts::from_args();

    let loader = match opts.loader {
        LoaderType::PostgreSQL => {
            let pg_opts = opts.pg_opts;

            let config = postgresql::Config {
                hostname: pg_opts.hostname,
                database: pg_opts.database.unwrap(),
                username: pg_opts.username,
                password: pg_opts.password,
                schema: pg_opts.schema,
            };

            postgresql::Conn::new(&config)?
        }
    };

    let drawer = Dot;
    let schema = loader.load()?;
    let mut buf = stdout();
    drawer.write(&schema, &mut buf)?;
    Ok(())
}
