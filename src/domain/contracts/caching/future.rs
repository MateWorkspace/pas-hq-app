use std::{future::Future, pin::Pin};

/// A boxed cache operation future that can be used through trait objects.
pub type CacheFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
