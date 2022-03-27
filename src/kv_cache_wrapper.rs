use worker::kv::KvStore;

pub(crate) enum KvCacheWrapper {
    EmptyWrapper,
    RealWrapper(KvStore),
}

pub(crate) enum ApiRequest {
    GetAllEvents,
    GetAnEvent(u32),
    GetEventBorders(u32),
    GetCurrentEventIds(),
    GetAllCards,
    GetACard(u32),
}

impl KvCacheWrapper {
    pub(crate) fn new(kvs: Option<KvStore>) -> Self {
        match kvs {
            Some(kv) => KvCacheWrapper::RealWrapper(kv),
            None => KvCacheWrapper::EmptyWrapper,
        }
    }

    // pub(crate) async fn get(req: ApiRequest) ->
}
