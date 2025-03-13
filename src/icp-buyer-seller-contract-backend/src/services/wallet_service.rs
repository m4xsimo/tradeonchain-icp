use alloy::network::EthereumWallet;
use alloy::signers::Signer;
use alloy::transports::icp::RpcApi;
use alloy::{
    transports::icp::{EthSepoliaService,RpcService},
    eips::BlockNumberOrTag,
    signers::icp::IcpSigner,
    primitives::{Address, U256, address},
    providers::{Provider, ProviderBuilder},
    rpc::client::{ClientBuilder, IcpClient},
    sol,
    transports::icp::IcpConfig,
};

use crate::repositories::ApiError;


pub trait WalletService {
    async fn get_balance(&self, address: String) -> Result<String, ApiError>;
    async fn get_address(&self) -> Result<String, ApiError>;
    async fn get_balance_usdc(&self, address: Option<String>) -> Result<String, ApiError>;
    async fn transfer_usdc(&self, amount: u64, to: Address) -> Result<String, ApiError>;
}


// Codegen from ABI file to interact with the contract.
sol!(
    #[allow(missing_docs, clippy::too_many_arguments)]
    #[sol(rpc)]
    USDC,
    "abi/USDC.json"
);

static USDC_CONTRACT_ADDRESS: &'static str = "1c7d4b196cb0c7b01d743fbc6116a902379c7238";
static LINK_CONTRACT_ADDRESS: &'static str = "779877A7B0D9E8603169DdbD7836e478b4624789";
static CONTRACT_ADDRESS: &'static str = LINK_CONTRACT_ADDRESS;

const SEPOLIA_CHAIN_ID: u64 = 11155111;
const BASE_CHAIN_ID: u64 = 1;


fn get_rpc_service_sepolia() -> RpcService {
    //To deploy an proxy on CloudFlare for Alchemy as provider,
    // fork and deploy this example: https://github.com/c-atts/catts-evm-rpc-proxy
    RpcService::Custom(RpcApi {
        url: "https://catts-evm-proxy-2.aledema.workers.dev/eth-sepolia".to_string(),
        headers: None,
    })
}

fn get_rpc_service_base() -> RpcService {
    // Uncomment to use EVM RPC Canister instead of RPC proxy
    // RpcService::BaseMainnet(L2MainnetService::Alchemy)

    RpcService::Custom(RpcApi {
        url: "https://catts-evm-proxy-2.aledema.workers.dev/base-mainnet".to_string(),
        headers: None,
    })
}

fn get_ecdsa_key_name() -> String {
    #[allow(clippy::option_env_unwrap)]
    let dfx_network = option_env!("DFX_NETWORK").unwrap();
    match dfx_network {
        "local" => "dfx_test_key".to_string(),
        "ic" => "key_1".to_string(),
        _ => panic!("Unsupported network."),
    }
}

async fn create_icp_signer() -> IcpSigner {
    let ecdsa_key_name = get_ecdsa_key_name();
    IcpSigner::new(vec![], &ecdsa_key_name, None).await.unwrap()
}


pub struct WalletServiceImpl {
    use_testnet: bool,
}

impl WalletServiceImpl {
    pub fn new(use_testnet: bool) -> Self {
        Self {use_testnet}
    }

    pub fn with_testnet(&mut self, use_testnet: bool) {
        self.use_testnet = use_testnet;
    }
}

impl Default for WalletServiceImpl {
    fn default() -> Self {
        Self::new(false)
    }
}

impl WalletService for WalletServiceImpl {

    /// Get the Ethereum address of the backend canister.
    async fn get_address(&self) -> Result<String, ApiError> {
        let ecdsa_key_name = get_ecdsa_key_name();
        let signer = IcpSigner::new(vec![], &ecdsa_key_name, None).await.unwrap();
        let address = signer.address();
        Ok(address.to_string())
    }


    async fn get_balance(&self, address: String) -> Result<String, ApiError> {
        let address = address.parse::<Address>().map_err(|e| ApiError::internal(e.to_string().as_str()))?;
        let rpc_service = if self.use_testnet {
            get_rpc_service_sepolia()
        } else {
            get_rpc_service_base()
        };
        
        let config = IcpConfig::new(rpc_service);
        let provider = ProviderBuilder::new().on_icp(config);
        let result = provider.get_balance(address).await;
        match result {
            Ok(balance) => Ok(balance.to_string()),
            Err(e) => Err(ApiError::internal(e.to_string().as_str())),
        }
    }

    /// Request the balance of an ETH account.
    async fn get_balance_usdc(&self, address: Option<String>) -> Result<String, ApiError> {
        let address = match address {
            Some(val) => val,
            None => {
                let signer = create_icp_signer().await;
                signer.address().to_string()
            }
        };
        let address = address.parse::<Address>().map_err(|e| ApiError::internal(e.to_string().as_str()))?;
        let rpc_service = if self.use_testnet {
            get_rpc_service_sepolia()
        } else {
            get_rpc_service_base()
        };
        let config = IcpConfig::new(rpc_service);
        let provider = ProviderBuilder::new().on_icp(config);

        let contract = USDC::new(
            CONTRACT_ADDRESS.parse::<Address>().map_err(|e| ApiError::internal(e.to_string().as_str()))?,
            provider,
        );


        let result = contract.balanceOf(address).call().await;
        match result {
            Ok(balance) => Ok(balance._0.to_string()),
            Err(e) => Err(ApiError::internal(e.to_string().as_str())),
        }
    }

    async fn transfer_usdc(&self, amount: u64, to: Address) -> Result<String, ApiError> {
        // Setup signer
        let signer = create_icp_signer().await;
        let address = signer.address();

        // Setup provider
        let wallet = EthereumWallet::from(signer);
        let rpc_service = if self.use_testnet {
            get_rpc_service_sepolia()
        } else {
            get_rpc_service_base()
        };

        let config = IcpConfig::new(rpc_service)
        .set_max_response_size(2000);
    
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_icp(config);

        let contract = USDC::new(
            CONTRACT_ADDRESS.parse::<Address>().map_err(|e| ApiError::internal(e.to_string().as_str()))?,
            provider.clone(),
        );


        let chain_id = if self.use_testnet {
            SEPOLIA_CHAIN_ID
        } else {
            BASE_CHAIN_ID
        };
        match contract
            .transfer(to, U256::from(amount))
            .chain_id(chain_id)
            .from(address)
            .send()
            .await
        {
            Ok(builder) => {
                let node_hash = *builder.tx_hash();
                let tx_response = provider.get_transaction_by_hash(node_hash).await.unwrap();

                match tx_response {
                    Some(tx) => {
                        Ok(format!("{:?}", tx))
                    }
                    None => Err(ApiError::internal("Could not get transaction.")),
                }
            }
            Err(e) => Err(ApiError::internal(e.to_string().as_str())),
        }
    }

}
