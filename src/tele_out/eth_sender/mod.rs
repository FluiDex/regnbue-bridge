use super::types::{ContractCall, ProofData};
use crate::contracts;
use crate::storage::PoolType;
use crate::tele_out::Settings;
use crossbeam_channel::Receiver;
use ethers::{
    abi::Abi,
    contract::Contract,
    providers::{Http, Provider},
    types::Address,
};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct EthSender {
    connpool: PoolType,
    contract: Contract<Provider<Http>>,
}

impl EthSender {
    pub fn from_config_with_pool(config: &Settings, connpool: PoolType) -> Result<Self, anyhow::Error> {
        let address = config.contract_address.parse::<Address>()?;
        let abi: Abi = contracts::get_abi(&config.contract_abi_file_path)?;
        let client = Provider::<Http>::try_from(config.web3_url.as_str())?; // assume wallet inside
        let contract = Contract::new(address, abi, client);

        Ok(Self { connpool, contract })
    }

    pub async fn run(&self, rx: Receiver<ContractCall>) {
        for call in rx.iter() {
            log::debug!("{:?}", call);
            let action = match call {
                ContractCall::SubmitProof(data) => self.submit_proof(data),
            };
            if let Err(e) = action.await {
                log::error!("{:?}", e);
            };

            // TODO: save to db
        }
    }

    pub async fn submit_proof(&self, data: ProofData) -> Result<(), anyhow::Error> {
        Ok(())
    }
}
