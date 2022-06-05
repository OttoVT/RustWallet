use std::fmt::Display;

use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize,Debug)]
pub enum BlockchainNetwork{
    EthereumMainnet,
    EthereuRopsten,
    EthereumGoerli,
    PolygonMainnet,
    PolygonMumbai,
    BinanceMainnet,
    BinanceTestnet,
}