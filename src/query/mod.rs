use std::env::var;

use crate::Action;
use libsql::{params, Connection, Database};

fn establish_connection() -> Connection {
    let db_url = var("DATABASE").unwrap();
    let auth_token = var("TURSO_AUTH_TOKEN").unwrap();
    let db = Database::open_remote(db_url, auth_token).expect("Failed to open remote turso db");
    db.connect().expect("Connection_failed")
}

#[derive(Clone)]
pub struct Query {
    pub conn: Connection,
}

impl Query {
    pub fn new() -> Self {
        Self {
            conn: establish_connection(),
        }
    }

    /// Creates a new table `russenger_user` in the database.
    ///
    /// This method returns a boolean indicating whether the operation was successful.
    ///
    /// # Returns
    ///
    /// * `bool`: Whether the operation was successful.
    pub async fn migrate(&self) -> bool {
        let sql = "
            create table russenger_user (
                facebook_user_id varchar(40) primary key unique,
                action varchar(20)
            );";
        if let Err(err) = self.conn.execute(sql, ()).await {
            eprintln!("Error on create table: {err}");
            false
        } else {
            true
        }
    }

    /// Inserts a new user into the `russenger_user` table.
    ///
    /// This method takes a user ID as an argument and returns a boolean indicating whether the operation was successful.
    ///
    /// # Arguments
    ///
    /// * `user_id`: The user ID of the new user.
    ///
    /// # Returns
    ///
    /// * `bool`: Whether the operation was successful.
    pub async fn create(&self, user_id: &str) -> bool {
        let sql = "insert into russenger_user (facebook_user_id, action) values (?1, ?2)";
        self.conn
            .execute(sql, params![user_id, "Main"])
            .await
            .is_ok()
    }
    /// Updates the action of a user in the `russenger_user` table.
    ///
    /// This method takes a user ID and an action as arguments and returns a boolean indicating whether the operation was successful.
    ///
    /// # Arguments
    ///
    /// * `user_id`: The user ID of the user whose action is to be updated.
    /// * `action`: The new action for the user.
    ///
    /// # Returns
    ///
    /// * `bool`: Whether the operation was successful.
    ///
    /// # Examples
    ///
    /// Updating the action of a user:
    ///
    /// ```rust
    /// use russenger::prelude::*;
    ///
    /// create_action!(Main, |res: Res, req: Req| async move {
    ///    req.query.set_action(&req.user, NextAction).await;
    /// });
    ///
    /// create_action!(NextAction, |res: Res, req: Req| async move {});
    ///
    /// russenger_app!(Main, Action);
    ///
    /// # Database Queries
    ///
    /// This method executes different SQL queries based on the type of the database:
    ///
    /// * For MySQL: `"update russenger_user set action=? where facebook_user_id=?"`
    /// * For SQLite: `"update russenger_user set action=$1 where facebook_user_id=$2"`
    /// * For Postgres: `"update russenger_user set action=$1 where facebook_user_id=$2"`
    pub async fn set_action<A: Action>(&self, user_id: &str, action: A) -> bool {
        let sql = "update russenger_user set action=?1 where facebook_user_id=?2";
        self.conn
            .execute(sql, params![action.path(), user_id])
            .await
            .is_ok()
    }

    /// Retrieves the action of a user from the `russenger_user` table.
    ///
    /// This method takes a user ID as an argument and returns the action of the user if it exists.
    ///
    /// # Arguments
    ///
    /// * `user_id`: The user ID of the user whose action is to be retrieved.
    ///
    /// # Returns
    ///
    /// * `Option<String>`: The action of the user if it exists, or `None` if it doesn't.
    ///
    /// # Database Queries
    ///
    /// This method executes different SQL queries based on the type of the database:
    ///
    /// * For MySQL: `"select action from russenger_user where facebook_user_id=?"`
    /// * For SQLite: `"select action from russenger_user where facebook_user_id=$1"`
    /// * For Postgres: `"select action from russenger_user where facebook_user_id=$1"`
    pub async fn get_action(&self, user_id: &str) -> Option<String> {
        let sql = "select action from russenger_user where facebook_user_id=?1";
        match self.conn.query(sql, params![user_id]).await {
            Ok(mut rows) => {
                if let Ok(row) = rows.next() {
                    row.unwrap().get(0).ok()
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }
}
