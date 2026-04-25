/// wmi.rs — Helper anti-freeze pour les requêtes WMI/blocking
///
/// `wmi_timeout` wraps une closure bloquante dans spawn_blocking + tokio::time::timeout.
/// Toute commande WMI doit passer par ce wrapper pour éviter les freezes
/// causés par des appels WMI qui ne répondent pas (drivers corrompus, services morts, etc.)
use crate::error::NiTriTeError;

/// Timeout global pour les opérations WMI bloquantes (30 secondes).
pub const WMI_TIMEOUT_SECS: u64 = 30;

/// Exécute `f` dans un thread bloquant avec timeout de 30s.
/// Retourne `NiTriTeError::System("WMI timeout")` si dépassé.
pub async fn wmi_timeout<T, F>(f: F) -> Result<T, NiTriTeError>
where
    F: FnOnce() -> Result<T, NiTriTeError> + Send + 'static,
    T: Send + 'static,
{
    tokio::time::timeout(
        std::time::Duration::from_secs(WMI_TIMEOUT_SECS),
        tokio::task::spawn_blocking(f),
    )
    .await
    .map_err(|_| NiTriTeError::System(format!("WMI timeout ({}s)", WMI_TIMEOUT_SECS)))?
    .map_err(|e| NiTriTeError::System(e.to_string()))?
}
