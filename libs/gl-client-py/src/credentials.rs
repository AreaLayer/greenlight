use crate::runtime::exec;
use crate::scheduler::Scheduler;
use crate::signer::Signer;
use gl_client::credentials::{self, NodeIdProvider, RuneProvider, TlsConfigProvider};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

pub type PyCredentials = UnifiedCredentials<credentials::Nobody, credentials::Device>;

#[derive(Clone)]
pub enum UnifiedCredentials<T, R>
where
    T: TlsConfigProvider,
    R: TlsConfigProvider + RuneProvider + NodeIdProvider + Clone,
{
    Nobody(T),
    Device(R),
}

impl<T, R> UnifiedCredentials<T, R>
where
    T: TlsConfigProvider,
    R: TlsConfigProvider + RuneProvider + NodeIdProvider + Clone,
{
    pub fn ensure_nobody(&self) -> Result<()> {
        if let Self::Nobody(_) = self {
            Ok(())
        } else {
            Err(credentials::Error::IsIdentityError(
                "credentials are not of type nobody".to_string(),
            ))?
        }
    }

    pub fn ensure_device(&self) -> Result<()> {
        if let Self::Device(_) = self {
            Ok(())
        } else {
            Err(credentials::Error::IsIdentityError(
                "credentials are not of type device".to_string(),
            ))?
        }
    }
}

impl<T, R> TlsConfigProvider for UnifiedCredentials<T, R>
where
    T: TlsConfigProvider,
    R: TlsConfigProvider + RuneProvider + NodeIdProvider + Clone,
{
    fn tls_config(&self) -> gl_client::tls::TlsConfig {
        match self {
            UnifiedCredentials::Nobody(n) => n.tls_config(),
            UnifiedCredentials::Device(d) => d.tls_config(),
        }
    }
}

impl<T, R> RuneProvider for UnifiedCredentials<T, R>
where
    T: TlsConfigProvider,
    R: TlsConfigProvider + RuneProvider + NodeIdProvider + Clone,
{
    fn rune(&self) -> String {
        match self {
            UnifiedCredentials::Nobody(_) => panic!(
                "can not provide rune from nobody credentials! something really bad happened."
            ),
            UnifiedCredentials::Device(d) => d.rune(),
        }
    }
}

impl<T, R> NodeIdProvider for UnifiedCredentials<T, R>
where
    T: TlsConfigProvider,
    R: TlsConfigProvider + RuneProvider + NodeIdProvider + Clone,
{
    fn node_id(&self) -> credentials::Result<Vec<u8>> {
        match self {
            UnifiedCredentials::Nobody(_) => panic!(
                "can not provide node_id from nobody credentials! something really bad happened."
            ),
            UnifiedCredentials::Device(d) => d.node_id(),
        }
    }
}

#[pyclass]
#[derive(Clone)]
pub struct Credentials {
    pub inner: PyCredentials,
}

#[pymethods]
impl Credentials {
    #[new]
    pub fn new() -> Self {
        let inner = UnifiedCredentials::Nobody(gl_client::credentials::Nobody::default());
        log::debug!("Created NOBODY credentials");
        Self { inner }
    }

    #[staticmethod]
    pub fn nobody_with(cert: &[u8], key: &[u8]) -> Self {
        let inner = UnifiedCredentials::Nobody(gl_client::credentials::Nobody::with(cert, key));
        log::debug!("Created NOBODY credentials");
        Self { inner }
    }

    #[staticmethod]
    pub fn from_path(path: &str) -> Self {
        let inner = UnifiedCredentials::Device(gl_client::credentials::Device::from_path(path));
        log::debug!("Created device credentials");
        Self { inner }
    }

    #[staticmethod]
    pub fn from_bytes(data: &[u8]) -> Self {
        let inner = UnifiedCredentials::Device(gl_client::credentials::Device::from_bytes(data));
        log::debug!("Created device credentials");
        Self { inner }
    }

    #[staticmethod]
    pub fn from_parts(cert: &[u8], key: &[u8], rune: &str) -> Self {
        let inner =
            UnifiedCredentials::Device(gl_client::credentials::Device::with(cert, key, rune));
        Self { inner }
    }

    pub fn upgrade(&self, scheduler: &Scheduler, signer: &Signer) -> Result<Credentials> {
        match &self.inner {
            UnifiedCredentials::Nobody(_) => Err(credentials::Error::IsIdentityError(
                "can not upgrade nobody credentials".to_string(),
            ))?,
            UnifiedCredentials::Device(creds) => match &scheduler.inner {
                crate::scheduler::UnifiedScheduler::Unauthenticated(u) => {
                    let d = exec(async move { creds.clone().upgrade(u, &signer.inner).await })?;
                    let inner = UnifiedCredentials::Device(d);
                    Ok(Self { inner })
                }
                crate::scheduler::UnifiedScheduler::Authenticated(a) => {
                    let d = exec(async move { creds.clone().upgrade(a, &signer.inner).await })?;
                    let inner = UnifiedCredentials::Device(d);
                    Ok(Self { inner })
                }
            },
        }
    }

    pub fn to_bytes<'a>(&self, py: Python<'a>) -> Result<&'a PyBytes> {
        match &self.inner {
            UnifiedCredentials::Nobody(_) => Err(credentials::Error::IsIdentityError(
                "can not convert nobody into bytes".to_string(),
            ))?,
            UnifiedCredentials::Device(d) => Ok(PyBytes::new(py, &d.to_bytes()[..])),
        }
    }

    pub fn ensure_device(&self) -> Result<()> {
        self.inner.ensure_device()
    }

    pub fn ensure_nobody(&self) -> Result<()> {
        self.inner.ensure_nobody()
    }

    pub fn node_id(&self) -> Result<Vec<u8>> {
        Ok(self.inner.node_id()?)
    }

    pub fn with_ca(&self, ca: &[u8]) -> Self {
        match &self.inner {
            UnifiedCredentials::Nobody(creds) => {
                let n = creds.clone().with_ca(ca);
                let inner = UnifiedCredentials::Nobody(n);
                Self { inner }
            }
            UnifiedCredentials::Device(creds) => {
                let d = creds.clone().with_ca(ca);
                let inner = UnifiedCredentials::Device(d);
                Self { inner }
            },
        }
    }
}

type Result<T, E = ErrorWrapper> = std::result::Result<T, E>;

#[derive(thiserror::Error, Debug)]
pub enum ErrorWrapper {
    #[error("{}", .0)]
    CredentialsError(#[from] credentials::Error),
}

impl From<ErrorWrapper> for pyo3::PyErr {
    fn from(value: ErrorWrapper) -> Self {
        PyErr::new::<PyValueError, _>(value.to_string())
    }
}
