use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::{Duration, Instant};

pub async fn subscriber(queue: usize, host: String) {
    let mut mqttoptions = MqttOptions::new("subscriber", host, 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, queue);
    client
        .subscribe("benchmark", QoS::AtMostOnce)
        .await
        .unwrap();

    let mut counter = 0;
    let mut last_print = Instant::now();

    while let Ok(_notification) = eventloop.poll().await {
        counter += 1;

        if last_print.elapsed() >= Duration::from_secs(1) {
            println!("Events per second: {}", counter);
            counter = 0;
            last_print = Instant::now();
        }
    }
}
