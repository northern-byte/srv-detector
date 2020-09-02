use std::future::Future;
use futures::future::JoinAll;
use std::slice::Iter;
use std::sync::Arc;

pub trait MultiSpawn<T, V> {
    fn spawn_and_join(self, f: fn(Arc<V>) -> T) -> JoinAll<tokio::task::JoinHandle<T::Output>>
        where
            T: Future + Send + 'static,
            T::Output: Send + 'static;
}

impl<T, V> MultiSpawn<T, V> for Iter<'_, Arc<V>> {
    fn spawn_and_join(self, f: fn(Arc<V>) -> T) -> JoinAll<tokio::task::JoinHandle<T::Output>>
        where
            T: Future + Send + 'static,
            T::Output: Send + 'static {
        let tasks = self.map(|u| tokio::spawn(f(u.clone()))).collect::<Vec<_>>();
        futures::future::join_all(tasks)
    }
}