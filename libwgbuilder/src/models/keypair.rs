//! Keypair model
//!
//! Keypairs are used by clients and the server for authentication and encryption
use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::Result;
use diesel::{Insertable, PgConnection, Queryable, RunQueryDsl};

use crate::schema::keypairs;
use crate::Error;

/// Keypair from the database
#[derive(Queryable)]
pub struct Keypair {
    pub id: i32,
    pub private_key: String,
    pub public_key: String,
}

/// A Keypair that is not created in the database yet
#[derive(Insertable)]
#[diesel(table_name = keypairs)]
pub struct NewKeypair<'a> {
    private_key: &'a str,
    public_key: &'a str,
}

impl NewKeypair<'_> {
    /// Returns a new keypair object that is ready for inserting into the database
    pub fn new<'a>(private_key: &'a str, public_key: &'a str) -> NewKeypair<'a> {
        NewKeypair {
            private_key: &private_key,
            public_key: &public_key,
        }
    }

    /// Creates a new keypair in the database
    pub fn create(self, conn: &mut PgConnection) -> Result<Keypair> {
        diesel::insert_into(keypairs::table)
            .values(&self)
            .get_result(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not create keypair in database")
            })
    }

    /// Generates a new keypair and inserts it into the database
    pub fn generate<'a>(connection: &mut PgConnection) -> Result<Keypair> {
        // Generate private key
        let command_privkey = Command::new("wg")
            .arg("genkey")
            .output()
            .map_err(|e| anyhow::Error::from(e).context("Failed to generate private key"))?
            .stdout;
        let private_key = std::str::from_utf8(&command_privkey)
            .map_err(|e| anyhow::Error::from(e).context("Could not parse private key"))?
            .replace("\n", "");

        // Generate public key
        let pubkey_command = Command::new("wg")
            .arg("pubkey")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| anyhow::Error::from(e).context("Failed to generate public key"))?;
        let stdin = pubkey_command
            .stdin
            .as_ref()
            .unwrap()
            .write_all(private_key.as_bytes());
        if let Err(e) = stdin {
            panic!("Could not read: {}", e);
        }

        let pubkey_output = pubkey_command
            .wait_with_output()
            .expect("Did not get a response from wg pubkey");
        let public_key = std::str::from_utf8(&pubkey_output.stdout)
            .expect("Could not parse public key")
            .replace("\n", "");

        NewKeypair::create(NewKeypair::new(&private_key, &public_key), connection)
    }
}
