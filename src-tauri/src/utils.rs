use serde::Serialize;
use std::fmt::Display;

#[derive(Serialize)]
pub struct CommandResult<T> {
    pub success: bool,
    pub value: Option<T>,
    pub error: Option<String>,
}

impl<T> CommandResult<T> {
    #[inline]
    pub fn ok(v: T) -> Self {
        Self {
            success: true,
            value: Some(v),
            error: None,
        }
    }
    #[inline]
    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            value: None,
            error: Some(msg.into()),
        }
    }
}

impl<T> From<Result<T, String>> for CommandResult<T> {
    fn from(r: Result<T, String>) -> Self {
        match r {
            Ok(v) => CommandResult::ok(v),
            Err(e) => CommandResult::err(e),
        }
    }
}

pub async fn run_blocking<F, T, E>(f: F) -> CommandResult<T>
where
    F: FnOnce() -> Result<T, E> + Send + 'static,
    T: Send + 'static + Serialize,
    E: Display + 'static,
{
    match tokio::task::spawn_blocking(move || f().map_err(|e| e.to_string())).await {
        Ok(res) => CommandResult::from(res),
        Err(e) => CommandResult::err(format!("Tokio join error: {e}")),
    }
}

#[macro_export]
macro_rules! cmd_blocking {
    ($e:expr) => {{
        crate::utils::run_blocking(move || $e).await
    }};
}
