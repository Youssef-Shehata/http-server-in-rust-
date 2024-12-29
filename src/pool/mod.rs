use tokio::net::TcpStream;

use std::collections::VecDeque;
use tokio::sync::{Mutex, Semaphore};

use std::sync::Arc;

#[derive(Clone)]
pub struct ConnectionPool {
    connections: Arc<Mutex<VecDeque<TcpStream>>>, // Arc around Mutex

    semaphore: Arc<Semaphore>,
    max_size: usize,
}

impl ConnectionPool {
    pub fn new(max_size: usize) -> Self {
        ConnectionPool {
            connections: Arc::new(Mutex::new(VecDeque::with_capacity(max_size))),
            semaphore: Arc::new(Semaphore::new(max_size)),
            max_size,
        }
    }

    pub async fn acquire(&self) -> Option<TcpStream> {
        let _ = self.semaphore.acquire().await.ok()?;
        let mut connections = self.connections.lock().await;

        connections.pop_front()
    }

    pub async fn release(&self, conn: TcpStream) {
        let mut connections = self.connections.lock().await;
        if connections.len() < self.max_size {
            connections.push_back(conn);
            self.semaphore.add_permits(1);
        }
    }
}
