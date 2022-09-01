/*
    Appellation: server <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Server {
    pub host: String,
    pub port: u16,
}

impl Server {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }
    pub fn address(self) -> std::net::SocketAddr {
        std::net::SocketAddr::from(self.pieces())
    }
    pub fn pieces(self) -> ([u8; 4], u16) {
        let host: [u8; 4] = scsys::extract::Extractor::new('.', self.host.clone())
            .extract()
            .try_into()
            .ok()
            .unwrap();
        (host, self.port)
    }
}

impl std::fmt::Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "View the server locally at http://localhost:{}",
            self.port
        )
    }
}
