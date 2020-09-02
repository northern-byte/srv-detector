use std::future::Future;
use futures::future::JoinAll;
use std::slice::Iter;

pub trait MultiSpawn<T, V> {
    fn spawn_and_join(self, f: fn(V) -> T) -> JoinAll<tokio::task::JoinHandle<T::Output>>
        where
            V: Clone,
            T: Future + Send + 'static,
            T::Output: Send + 'static;
}

impl<T, V> MultiSpawn<T, V> for Iter<'_, V> {
    fn spawn_and_join(self, f: fn(V) -> T) -> JoinAll<tokio::task::JoinHandle<T::Output>>
        where
            V: Clone,
            T: Future + Send + 'static,
            T::Output: Send + 'static {
        let tasks = self.map(|u| tokio::spawn(f(u.clone()))).collect::<Vec<_>>();
        futures::future::join_all(tasks)
    }
}