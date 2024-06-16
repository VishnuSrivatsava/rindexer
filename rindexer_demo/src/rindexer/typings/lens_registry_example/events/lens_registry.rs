/// THIS IS A GENERATED FILE. DO NOT MODIFY MANUALLY.
///
/// This file was auto generated by rindexer - https://github.com/joshstevens19/rindexer.
/// Any manual changes to this file will be overwritten.
use super::lens_registry_abi_gen::rindexer_lens_registry_gen::{self, RindexerLensRegistryGen};
use ethers::{
    abi::Address,
    providers::{Http, Provider, RetryClient},
    types::{Bytes, H256},
};
use rindexer_core::{
    async_trait, generate_random_id,
    generator::event_callback_registry::{
        ContractInformation, EventCallbackRegistry, EventInformation, EventResult, FactoryDetails,
        FilterDetails, NetworkContract, TxInformation,
    },
    manifest::yaml::{Contract, ContractDetails},
    AsyncCsvAppender, FutureExt, PostgresClient,
};
use std::future::Future;
use std::path::Path;
use std::pin::Pin;
use std::{any::Any, sync::Arc};

pub type HandleLinkedData = rindexer_lens_registry_gen::HandleLinkedFilter;

#[derive(Debug)]
pub struct HandleLinkedResult {
    pub event_data: HandleLinkedData,
    pub tx_information: TxInformation,
}

pub type HandleUnlinkedData = rindexer_lens_registry_gen::HandleUnlinkedFilter;

#[derive(Debug)]
pub struct HandleUnlinkedResult {
    pub event_data: HandleUnlinkedData,
    pub tx_information: TxInformation,
}

pub type NonceUpdatedData = rindexer_lens_registry_gen::NonceUpdatedFilter;

#[derive(Debug)]
pub struct NonceUpdatedResult {
    pub event_data: NonceUpdatedData,
    pub tx_information: TxInformation,
}

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[async_trait]
trait EventCallback {
    async fn call(&self, events: Vec<EventResult>);
}

pub struct EventContext<TExtensions>
where
    TExtensions: Send + Sync,
{
    pub database: Arc<PostgresClient>,
    pub csv: Arc<AsyncCsvAppender>,
    pub extensions: Arc<TExtensions>,
}

// didn't want to use option or none made harder DX
// so a blank struct makes interface nice
pub struct NoExtensions {}
pub fn no_extensions() -> NoExtensions {
    NoExtensions {}
}

type HandleLinkedEventCallbackType<TExtensions> = Arc<
    dyn Fn(&Vec<HandleLinkedResult>, Arc<EventContext<TExtensions>>) -> BoxFuture<'_, ()>
        + Send
        + Sync,
>;

pub struct HandleLinkedEvent<TExtensions>
where
    TExtensions: Send + Sync,
{
    callback: HandleLinkedEventCallbackType<TExtensions>,
    context: Arc<EventContext<TExtensions>>,
}

impl<TExtensions> HandleLinkedEvent<TExtensions>
where
    TExtensions: Send + Sync,
{
    pub async fn new(
        callback: HandleLinkedEventCallbackType<TExtensions>,
        extensions: TExtensions,
    ) -> Self {
        let csv = AsyncCsvAppender::new("/Users/joshstevens/code/rindexer/rindexer_demo/./generated_csv/LensRegistry/lensregistry-handlelinked.csv".to_string());
        if !Path::new("/Users/joshstevens/code/rindexer/rindexer_demo/./generated_csv/LensRegistry/lensregistry-handlelinked.csv").exists() {
            csv.append_header(vec!["contract_address".into(), "handle_id".into(), "handle_collection".into(), "token_id".into(), "token_collection".into(), "transaction_executor".into(), "timestamp".into(), "tx_hash".into(), "block_number".into(), "block_hash".into()])
                .await
                .unwrap();
        }

        Self {
            callback,
            context: Arc::new(EventContext {
                database: Arc::new(PostgresClient::new().await.unwrap()),
                csv: Arc::new(csv),
                extensions: Arc::new(extensions),
            }),
        }
    }
}

#[async_trait]
impl<TExtensions> EventCallback for HandleLinkedEvent<TExtensions>
where
    TExtensions: Send + Sync,
{
    async fn call(&self, events: Vec<EventResult>) {
        let events_len = events.len();

        let result: Vec<HandleLinkedResult> = events
            .into_iter()
            .filter_map(|item| {
                item.decoded_data
                    .downcast::<HandleLinkedData>()
                    .ok()
                    .map(|arc| HandleLinkedResult {
                        event_data: (*arc).clone(),
                        tx_information: item.tx_information,
                    })
            })
            .collect();

        if result.len() == events_len {
            (self.callback)(&result, self.context.clone()).await;
        } else {
            panic!("HandleLinkedEvent: Unexpected data type - expected: HandleLinkedData")
        }
    }
}

pub enum LensRegistryEventType<TExtensions>
where
    TExtensions: 'static + Send + Sync,
{
    HandleLinked(HandleLinkedEvent<TExtensions>),
}

impl<TExtensions> LensRegistryEventType<TExtensions>
where
    TExtensions: 'static + Send + Sync,
{
    pub fn topic_id(&self) -> &'static str {
        match self {
            LensRegistryEventType::HandleLinked(_) => {
                "0xc33717b10968a153b6fa59edb9356cc86846bd3be6f416f4410338e25bb28d3c"
            }
        }
    }

    pub fn event_name(&self) -> &'static str {
        match self {
            LensRegistryEventType::HandleLinked(_) => "HandleLinked",
        }
    }

    pub fn contract_information(&self) -> Contract {
        Contract {
            name: "LensRegistry".to_string(),
            details: vec![ContractDetails::new_with_address(
                "polygon".to_string(),
                "0xD4F2F33680FCCb36748FA9831851643781608844".to_string(),
                Some(56399431.into()),
                Some(56399432.into()),
                Some(1000),
            )],
            abi:
                "/Users/joshstevens/code/rindexer/rindexer_demo/abis/lens-registry-events-abi.json"
                    .to_string(),
            include_events: Some(vec!["HandleLinked".to_string()]),
            reorg_safe_distance: false,
            generate_csv: true,
        }
    }

    fn get_provider(&self, network: &str) -> Arc<Provider<RetryClient<Http>>> {
        if network == "polygon" {
            super::super::super::networks::get_polygon_provider()
        } else {
            panic!("Network not supported")
        }
    }

    fn contract(&self, network: &str) -> RindexerLensRegistryGen<Arc<Provider<RetryClient<Http>>>> {
        if network == "polygon" {
            let address: Address = "0xD4F2F33680FCCb36748FA9831851643781608844"
                .parse()
                .unwrap();
            RindexerLensRegistryGen::new(address, Arc::new(self.get_provider(network).clone()))
        } else {
            panic!("Network not supported");
        }
    }

    fn decoder(
        &self,
        network: &str,
    ) -> Arc<dyn Fn(Vec<H256>, Bytes) -> Arc<dyn Any + Send + Sync> + Send + Sync> {
        let contract = self.contract(network);

        match self {
            LensRegistryEventType::HandleLinked(_) => {
                Arc::new(move |topics: Vec<H256>, data: Bytes| {
                    match contract.decode_event::<HandleLinkedData>("HandleLinked", topics, data) {
                        Ok(filter) => Arc::new(filter) as Arc<dyn Any + Send + Sync>,
                        Err(error) => Arc::new(error) as Arc<dyn Any + Send + Sync>,
                    }
                })
            }
        }
    }

    pub fn register(self, registry: &mut EventCallbackRegistry) {
        let topic_id = self.topic_id();
        let event_name = self.event_name();
        let contract_information = self.contract_information();
        let contract = ContractInformation {
            name: contract_information.name,
            details: contract_information
                .details
                .iter()
                .map(|c| NetworkContract {
                    id: generate_random_id(10),
                    network: c.network.clone(),
                    provider: self.get_provider(&c.network),
                    decoder: self.decoder(&c.network),
                    indexing_contract_setup: c.indexing_contract_setup(),
                    start_block: c.start_block,
                    end_block: c.end_block,
                    polling_every: c.polling_every,
                })
                .collect(),
            abi: contract_information.abi,
            reorg_safe_distance: contract_information.reorg_safe_distance,
        };

        let callback: Arc<dyn Fn(Vec<EventResult>) -> BoxFuture<'static, ()> + Send + Sync> =
            match self {
                LensRegistryEventType::HandleLinked(event) => {
                    let event = Arc::new(event);
                    Arc::new(move |result| {
                        let event = event.clone();
                        async move { event.call(result).await }.boxed()
                    })
                }
            };

        registry.register_event(EventInformation {
            indexer_name: "LensRegistryExample".to_string(),
            event_name: event_name.to_string(),
            topic_id: topic_id.to_string(),
            contract,
            callback,
        });
    }
}
