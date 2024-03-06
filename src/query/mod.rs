use std::env::var;

use libsql::{params, Connection, Database};

fn establish_connection() -> Connection {
    let db_url = var("TURSO_DABASE_URL").unwrap();
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

    pub async fn create(&self, user_id: &str) -> bool {
        let sql = "insert into russenger_user (facebook_user_id, action) values (?1, ?2)";
        self.conn
            .execute(sql, params![user_id, "Main"])
            .await
            .is_ok()
    }

    pub async fn set_action(&self, user_id: &str, action: &str) -> bool {
        let sql = "update russenger_user set action=?1 where facebook_user_id=?2";
        self.conn
            .execute(sql, params![action, user_id])
            .await
            .is_ok()
    }

    pub async fn get_action(&self, user_id: &str) -> Option<String> {
        let sql = "select action from russenger_user where facebook_user_id=?1";
        match self.conn.query(sql, params![user_id]).await {
            Ok(mut rows) => {
                if let Ok(row) = rows.next() {
                    row.unwrap().get(0).unwrap_or_default()
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    pub async fn reset_action(&self, user_id: &str) -> bool {
        self.set_action(user_id, "Main").await
    }
}
