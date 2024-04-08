mod health_check;
mod subscriptions;
mod subscriptions_confirm;
mod newsletters;
mod home;
mod login_mod;

pub use health_check::health_check_endpoint;
pub use subscriptions::*;
pub use subscriptions_confirm::*;
pub use newsletters::*;
pub use home::*;
pub use login_mod::login_form;
pub use login_mod::login;

pub fn error_chain_fmt(
    e: &impl std::error::Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}
