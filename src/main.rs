#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod action;
pub mod app;
pub mod cli;
pub mod components;
pub mod config;
pub mod database;
pub mod focus;
pub mod tui;
pub mod ui;
pub mod utils;
pub mod vim;

use std::{
  io::{self, Write},
  str::FromStr,
};

use clap::Parser;
use cli::Cli;
use color_eyre::eyre::{self, Result};
use sqlx::{postgres::PgConnectOptions, Connection, Database, Executor, Pool, Postgres};

use crate::{
  app::App,
  utils::{initialize_logging, initialize_panic_handler, version},
};

async fn tokio_main() -> Result<()> {
  initialize_logging()?;

  initialize_panic_handler()?;

  let args = Cli::parse();
  if let Some(db) = args.database.as_deref() {
    if db == "postgres" {
      let connection_opts = build_connection_opts::<Postgres>(args.clone())?;
      let mut app = App::<'_, Postgres>::new(connection_opts)?;
      app.run().await?;
    }
  }
  Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
  if let Err(e) = tokio_main().await {
    eprintln!("{} error: Something went wrong", env!("CARGO_PKG_NAME"));
    Err(e)
  } else {
    Ok(())
  }
}

// sqlx defaults to reading from environment variables if no inputs are provided
fn build_connection_opts<DB: Database>(args: Cli) -> Result<<DB::Connection as Connection>::Options>
where
  DB: Database + database::ValueParser,
  DB::QueryResult: database::HasRowsAffected,
  for<'c> <DB as sqlx::Database>::Arguments<'c>: sqlx::IntoArguments<'c, DB>,
  for<'c> &'c mut DB::Connection: Executor<'c, Database = DB>,
{
  match args.connection_url {
    Some(url) => Ok(<DB::Connection as Connection>::Options::from_str(&url)?),
    None => {
      // let mut opts = <DB::Connection as Connection>::Options::new();

      // if let Some(user) = args.user {
      //   opts = opts.username(&user);
      // } else {
      //   let mut user: String = String::new();
      //   print!("username: ");
      //   io::stdout().flush().unwrap();
      //   io::stdin().read_line(&mut user).unwrap();
      //   user = user.trim().to_string();
      //   if !user.is_empty() {
      //     opts = opts.username(&user);
      //   }
      // }

      // if let Some(password) = args.password {
      //   opts = opts.password(&password);
      // } else {
      //   let mut password = rpassword::prompt_password(format!("password for user {}: ", opts.get_username())).unwrap();
      //   password = password.trim().to_string();
      //   if !password.is_empty() {
      //     opts = opts.password(&password);
      //   }
      // }

      // if let Some(host) = args.host {
      //   opts = opts.host(&host);
      // } else {
      //   let mut host: String = String::new();
      //   print!("host (ex. localhost): ");
      //   io::stdout().flush().unwrap();
      //   io::stdin().read_line(&mut host).unwrap();
      //   host = host.trim().to_string();
      //   if !host.is_empty() {
      //     opts = opts.host(&host);
      //   }
      // }

      // if let Some(port) = args.port {
      //   opts = opts.port(port);
      // } else {
      //   let mut port: String = String::new();
      //   print!("port (ex. 5432): ");
      //   io::stdout().flush().unwrap();
      //   io::stdin().read_line(&mut port).unwrap();
      //   port = port.trim().to_string();
      //   if !port.is_empty() {
      //     opts = opts.port(port.parse()?);
      //   }
      // }

      // if let Some(database) = args.database {
      //   opts = opts.database(&database);
      // } else {
      //   let mut database: String = String::new();
      //   print!("database (ex. postgres): ");
      //   io::stdout().flush().unwrap();
      //   io::stdin().read_line(&mut database).unwrap();
      //   database = database.trim().to_string();
      //   if !database.is_empty() {
      //     opts = opts.database(&database);
      //   }
      // }

      // Ok(opts)
      Err(eyre::Report::msg("Not implemented"))
    },
  }
}
