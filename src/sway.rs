use {
    async_stream::stream,
    futures_core::Stream,
    swayipc_async as sway,
    tokio::time::{self, Duration},
};

pub use sway::{Connection, Error, Event, EventType, Node, NodeType};

pub async fn new_stream() -> Result<impl Stream<Item = ()>, Error> {
    let c = Connection::new().await?;
    Ok(stream! {
        loop {
            time::sleep(Duration::from_secs(1)).await;
            yield ();
        }
    })
}
