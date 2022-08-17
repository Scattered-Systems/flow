/*
    Appellation: accounts <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub trait AccountSpec {
    fn new(username: String, password: String) -> Self
        where
            Self: Sized;
    fn id(&self) -> scsys::bson::oid::ObjectId;
    fn name(&self) -> String;
    fn password(&self) -> String;
    fn slug(&self) -> String {
        self.name().clone().to_lowercase()
    }
    fn username(&self) -> String;
}

/// Defines a Web3 Account
#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct Web3Account {
    pub address: String,
    pub balance: usize,
    pub ensname: String,
}

impl Web3Account {
    fn constructor(address: String, balance: usize, ensname: String) -> Self {
        Self {
            address,
            balance,
            ensname,
        }
    }
    pub fn new(address: String, balance: usize, ensname: String) -> Self {
        Self::constructor(address, balance, ensname)
    }
    pub fn get_balance(&self) -> usize {
        todo!()
    }
}

impl Default for Web3Account {
    fn default() -> Self {
        Self::new(String::new(), 0, String::new())
    }
}

pub async fn get_account_balance(
    address: crate::Web3Address,
    client: crate::Web3Http,
) -> web3::Result<web3::types::U256> {
    let block_num = client.eth().block_number().await?;
    client
        .eth()
        .balance(
            address,
            Option::from(web3::types::BlockNumber::from(block_num)),
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account() {
        let actual = Web3Account::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
