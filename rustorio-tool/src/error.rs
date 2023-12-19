#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("http error")]
    Net(#[from] gloo_net::Error),
}
