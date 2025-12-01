use std::any::{TypeId, Any};
use std::sync::Arc;

#[derive(Clone)]
pub struct AnyValue {
    inner: Arc<dyn Any + Send + Sync + 'static>,
    id: TypeId
}

impl AnyValue {
    pub fn new<T>(value: T) -> Self
    where
        T: Any + Send + Sync + 'static,
    {
        AnyValue {
            inner: Arc::new(value),
            id: TypeId::of::<T>(),
        }
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Any + Send + Sync + 'static,
    {
        self.inner.downcast_ref::<T>()
    }

    pub fn downcast_into<T>(&mut self) -> Result<T, Self>
    where
        T: Any + Send + Sync + Clone + 'static,
    {
        let id = self.id;

        let value = match Arc::downcast::<T>(self.inner.clone()) {
            Ok(value) => value,
            Err(inner) => {
                return Err(AnyValue { inner, id })
            }
        };

        let value = Arc::try_unwrap(value).unwrap_or_else(|arc| (*arc).clone());
        Ok(value)
    }

    pub fn type_id(&self) -> TypeId {
        self.id
    }

}

impl std::fmt::Debug for AnyValue { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        f.debug_struct("AnyValue")
            .field("inner", &self.id)
            .finish()
    }
}

impl std::default::Default for AnyValue { 
    fn default() -> Self { 
        AnyValue::new(()) 
    } 
}