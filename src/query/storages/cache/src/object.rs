// Copyright 2022 Datafuse Labs.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::sync::Arc;

use common_exception::Result;
use opendal::Object;

use crate::ObjectCacheProvider;

/// Wrap with cache for the object write/read/remove trait.
/// If you want use cache, we should:
/// 1. create a cache provider(ByPassCache, MemoryBytesCache/MemoryItemsCache/FileCache)
/// 2. create the ObjectReaderWriter with the cache provider
/// 3. operation with object
pub struct ObjectReaderWriter<T> {
    cache: Arc<dyn ObjectCacheProvider<T>>,
}

impl<T> ObjectReaderWriter<T> {
    pub fn create(cache: Arc<dyn ObjectCacheProvider<T>>) -> ObjectReaderWriter<T> {
        Self { cache }
    }

    pub async fn read(&self, object: &Object, start: u64, end: u64) -> Result<Arc<T>> {
        self.cache.read_object(object, start, end).await
    }

    pub async fn write(&self, object: &Object, t: Arc<T>) -> Result<()> {
        self.cache.write_object(object, t).await
    }

    pub async fn remove(&self, object: &Object) -> Result<()> {
        self.cache.remove_object(object).await
    }
}
