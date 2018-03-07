use openssl::x509::{X509, X509Ref, X509StoreContext};
use openssl::x509::store::X509StoreRef;
use openssl::error::ErrorStack;
use openssl::stack::StackRef;

/// The `VerifyCertificate` trait is used by the verify certificate handler, to verify a
/// certificate.
pub trait VerifyCertificate {
    /// Will be called to verify the given certificate and certificates chain.
    ///
    /// # Result
    ///
    /// If the certificate could be verified, the function should return `Ok(())`, otherwise
    /// a `Err(ErrorStack)` is expected.
    fn verify(&mut self, cert: &X509Ref, chain: &StackRef<X509>) -> Result<(), ErrorStack>;
}

/// Provides a default implementation for verifying a certificate and certificates chain against
/// a `X509Store` with trusted certificates.
pub fn default_verify_certificate(
    cert: &X509Ref,
    chain: &StackRef<X509>,
    store: &X509StoreRef,
) -> Result<(), ErrorStack> {
    let mut context = X509StoreContext::new()?;
    context.verify_cert(store, cert, chain)
}