use rumqttc::{MqttOptions, AsyncClient, QoS};
use std::time::Duration;


pub async fn subscriber(queue: usize, host: String) {
    let mut mqttoptions = MqttOptions::new("subscriber", host, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, queue);
    client.subscribe("benchmark", QoS::AtMostOnce).await.unwrap();

    while let Ok(notification) = eventloop.poll().await {
        println!("Received = {:?}", notification);
    }
}