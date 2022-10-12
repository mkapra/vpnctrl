//! An user is allowed to query the API as an administrator or can retrieve his VPN configurations
use anyhow::Result;
use diesel::prelude::*;

use crate::Error;

/// User from the database
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub role: i32,
}

impl User {
    /// Searches for a user by its username
    ///
    /// # Returns
    /// Returns an `[Error::DatabaseObjectNotFound]` if the requested user was not found
    pub fn find_by_username(name: &str, conn: &mut PgConnection) -> Result<Self> {
        use crate::schema::users::{self, dsl::*};
        users::table
            .filter(username.eq(name))
            .first(conn)
            .map_err(|e| {
                anyhow::Error::from(e).context(Error::DatabaseObjectNotFound("username", 0))
            })
    }

    /// Updates the password for the given user
    ///
    /// # Returns
    /// The updated version of the user
    pub fn update_password(self, new_password: &str, conn: &mut PgConnection) -> Result<Self> {
        let hashed_password = bcrypt::hash(new_password, 8)
            .map_err(|e| anyhow::Error::from(e).context("Could not hash new password"))?;

        use crate::schema::users::{self, dsl::*};
        diesel::update(users::table)
            .set(password.eq(hashed_password))
            .get_result::<Self>(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not update new user password in database")
            })
    }
}

/// User that is not created in the database yet
#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser<'a> {
    username: &'a str,
    password: String,
    role: i32,
}

impl NewUser<'_> {
    /// Returns a  new user that is ready to be inserted into the database
    pub fn new<'a>(username: &'a str, password: &str, role: i32) -> NewUser<'a> {
        NewUser {
            username,
            role,
            password: bcrypt::hash(password, 8).expect("Could not generate password hash"),
        }
    }

    /// Creates the user in the database
    pub fn create(self, conn: &mut PgConnection) -> Result<User> {
        diesel::insert_into(crate::schema::users::table)
            .values(&self)
            .get_result(conn)
            .map_err(|e| {
                anyhow::Error::from(e)
                    .context(Error::Database)
                    .context("Could not create user in database")
            })
    }
}
