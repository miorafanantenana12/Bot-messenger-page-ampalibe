use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use tokio::sync::Mutex;

use super::{request::Req, response::Res};
use crate::response_models::data::Data;
use crate::response_models::payload::Payload;
use crate::response_models::quick_replies::{QuickReply, QuickReplyModel};

pub struct ActionLock {
    pub locked_users: Arc<Mutex<HashSet<String>>>,
}

impl ActionLock {
    pub async fn lock(&self, user: &str) -> bool {
        let mut locked_user = self.locked_users.lock().await;
        if !locked_user.contains(user) {
            locked_user.insert(user.to_string());
            true
        } else {
            false
        }
    }

    pub async fn unlock(&self, user: &str) {
        let mut locked_users = self.locked_users.lock().await;
        locked_users.remove(user);
    }
}

/// The `Action` trait defines the behavior of an action.
///
/// An action is a unit of work that the application can perform. Each action is associated with a path, and when a request is received with that path, the action's `execute` method is called.
///
/// # Methods
///
/// * `execute`: This method is called when a request is received with the action's path. It takes a `Res` and a `Req` as arguments, which represent the response and request respectively.
/// * `path`: This method returns the path associated with the action.
///
/// # Examples
///
/// ```rust
/// use russenger::prelude::*;
///
/// #[action]
/// async fn Greet(res: Res, req: Req) {
///     let message: String = req.data.get_value();
///     
///     if message == "Hello" {
///         res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await;
///     }
/// }
/// ```
#[async_trait::async_trait]
pub trait Action: Send + Sync {
    async fn execute(&self, res: Res, req: Req);

    fn path(&self) -> String;

    async fn next(&self, res: Res, req: Req) {
        let mut page = req.data.get_page().unwrap_or_default();
        page.next();
        let quick_reply: QuickReplyModel<'_> = QuickReplyModel::new(
            &req.user,
            "Navigation",
            vec![QuickReply::new(
                "Next",
                "",
                Payload {
                    path: self.path(),
                    data: Some(Data::new(req.data.get_value::<String>(), Some(page))),
                },
            )],
        );
        res.send(quick_reply).await;
    }
}

type ActionRegistryType = Arc<Mutex<HashMap<String, Box<dyn Action>>>>;

lazy_static::lazy_static! {
    /// `ACTION_REGISTRY` is a thread-safe map that stores all the actions available in the application.
    ///
    /// Each action is associated with a path, and when a request is received with that path, the corresponding action's `execute` method is called.
    ///
    /// The `ACTION_REGISTRY` is initialized with an empty map when the application starts, and actions are added to it using the `create_action!` macro.
    ///
    /// # Examples
    ///
    /// Adding an action to the `ACTION_REGISTRY`:
    /// ```rust
    /// use russenger::{ACTION_REGISTRY, Action};
    ///
    /// use russenger::prelude::*;
    ///
    /// #[action]
    /// async fn Main(res: Res, req: Req) {
    ///     let message: String = req.data.get_value();
    ///
    ///     if message == "Hello" {
    ///         res.send(TextModel::new(&req.user, "Hello, welcome to our bot!")).await;
    ///     }
    /// }
    ///
    /// #[russenger::main]
    /// async fn main() {
    ///     ACTION_REGISTRY.lock().await.insert(Main.path(), Box::new(Main));
    /// }
    /// ```
    pub static ref ACTION_REGISTRY: ActionRegistryType = Arc::new(Mutex::new(HashMap::new()));
    pub static ref ACTION_LOCK: ActionLock = ActionLock { locked_users: Arc::new(Mutex::new(HashSet::new()))};
}
