use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
#[derive(Clone, Debug)]
pub struct NodeLL {
    pub value: u64,
    next_node: Arc<Mutex<Box<Option<NodeLL>>>>,
}

impl NodeLL {
    pub fn new(val: u64) -> Self {
        NodeLL {
            value: val,
            next_node: Arc::new(Mutex::new(Box::new(None))),
        }
    }


    //if your wondering what this returns, dont
    //this is how you recurse with async fn's, this the fun side of async imo
    pub fn search<'a>(
        &'a self,
        val: u64,
    ) -> Pin<Box<dyn Future<Output = Option<Self>> + Send + 'a>> {
        Box::pin(async move {
            if val == self.value {
                return Some(self.clone());
            }

            let next_node = self.next_node.lock().await;
            if let Some(ref node) = **next_node {
                node.search(val).await
            } else {
                None
            }
        })
    }
    //pub async fn delete(&mut self, val: u64) {
    //deletes all but last node, cause its my linked list.
    //if let Some(n) = self.search(val).await {
    //    *self = n.next_node;
    //}
    //}
    
    pub fn insert<'a>(&'a self, val: u64) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            let mut next_node = self.next_node.lock().await;

            if let Some(ref next) = **next_node {
                //recurse to end of list
                next.insert(val).await;
            } else {
                // create the new node to be added at the end
                println!("wutang");
                let new_node = NodeLL {
                    value: val,
                    next_node: Arc::new(Mutex::new(Box::new(None))),
                };
                // Set the new node as the next node
                **next_node = Some(new_node);
            }
        })
    }

    pub fn traverse<'a>(&'a self) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move {
            let next = self.next_node.lock().await;
            println!("node{}", self.value);
            if let Some(ref node) = **next {
                node.traverse().await;
            }
        })
    }
}
