use std::{
    hash::{Hash, Hasher},
    ops::Deref,
    sync::{
        LazyLock,
        atomic::{AtomicU32, Ordering},
    },
};

use indexmap::Equivalent;
use leptos::web_sys::js_sys::{Object, Reflect};
use send_wrapper::SendWrapper;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ObjectId(u32);

static NEXT_ID: AtomicU32 = AtomicU32::new(0);
static ID_SYMBOL: LazyLock<SendWrapper<JsValue>> =
    LazyLock::new(move || SendWrapper::new(JsValue::symbol(Some("node_id"))));

impl ObjectId {
    pub fn for_object(object: &Object) -> Result<Self, JsValue> {
        let symbol = LazyLock::force(&ID_SYMBOL);
        match Reflect::get(object, symbol)? {
            value if value.is_undefined() => {
                let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
                let value = JsValue::from(id);
                Reflect::set(object, symbol, &value)?;
                Ok(ObjectId(id))
            }
            value => Ok(ObjectId(u64::try_from(value)? as u32)),
        }
    }

    pub fn for_value(value: &impl AsRef<Object>) -> Result<Self, JsValue> {
        Self::for_object(value.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct ObjectKey<T> {
    id: ObjectId,
    value: SendWrapper<T>,
}

impl<T: AsRef<Object>> ObjectKey<T> {
    pub fn new(value: T) -> Result<Self, JsValue> {
        ObjectId::for_object(value.as_ref()).map(move |id| Self {
            id,
            value: SendWrapper::new(value),
        })
    }
}

impl<T> PartialEq for ObjectKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for ObjectKey<T> {}

impl<T> Hash for ObjectKey<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> AsRef<T> for ObjectKey<T> {
    fn as_ref(&self) -> &T {
        &self.value
    }
}

impl<T> Deref for ObjectKey<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> Equivalent<ObjectKey<T>> for ObjectId {
    fn equivalent(&self, key: &ObjectKey<T>) -> bool {
        self == &key.id
    }
}
