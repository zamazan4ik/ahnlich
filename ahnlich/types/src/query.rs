use std::collections::HashSet;
use std::num::NonZeroUsize;

use crate::keyval::{SearchInput, StoreKey, StoreName, StoreValue};
use crate::metadata::MetadataKey;
use crate::predicate::PredicateCondition;
use crate::similarity::Algorithm;
use serde::Deserialize;
use serde::Serialize;

/// All possible queries for the server to respond to
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Query {
    Create {
        store: StoreName,
        dimension: NonZeroUsize,
        create_predicates: HashSet<MetadataKey>,
    },
    GetKey {
        keys: Vec<StoreKey>,
    },
    GetPred {
        store: StoreName,
        condition: PredicateCondition,
    },
    GetSimN {
        store: StoreName,
        closest_n: NonZeroUsize,
        input: SearchInput,
        algorithm: Algorithm,
        condition: Option<PredicateCondition>,
    },
    ReIndex {
        store: StoreName,
        predicates: HashSet<MetadataKey>,
    },
    DropIndexPred {
        store: StoreName,
        predicates: HashSet<MetadataKey>,
    },
    Set {
        store: StoreName,
        inputs: Vec<(StoreKey, StoreValue)>,
    },
    DelKey {
        store: StoreName,
        keys: Vec<StoreKey>,
    },
    DelPred {
        store: StoreName,
        condition: PredicateCondition,
    },
    DropStore {
        store: StoreName,
    },
    InfoServer,
    ListStores,
    ListClients,
    Ping,
}
