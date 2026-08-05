use cognite::models::instances::{InstanceId, Timeseries};
use cognite::time_series::{AddDatapoints, AddTimeSeries, DatapointDouble, DatapointsEnumType};
use cognite::{CogniteClient, IdentityOrInstance};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::time::Duration;

mod cog;
mod decode;

async fn subscribe(cognite_client: CogniteClient) {
    let mut mqttoptions = MqttOptions::new("rumqtt-async", "pi5", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("tibber", QoS::AtMostOnce).await.unwrap();
    loop {
        let notification = eventloop.poll().await.unwrap();
        if let Event::Incoming(msg) = notification
            && let Packet::Publish(msg) = msg
        {
            if let Ok(value) = decode::decode(msg.payload.to_vec()) {
                insert_datapoints(
                    cognite_client.clone(),
                    chrono::Utc::now().timestamp_millis(),
                    value as f64,
                )
                .await;
            }
        }
    }
}

async fn insert_datapoints(client: CogniteClient, time: i64, value: f64) {
    use cognite::Identity;
    use cognite::models::instances::{CogniteTimeseries, TimeSeriesType};
    use cognite::time_series::AddDmOrTimeSeries;
    client
        .time_series
        .insert_datapoints_create_missing(
            vec![AddDatapoints {
                id: IdentityOrInstance::InstanceId {
                    instance_id: InstanceId {
                        space: "Nibe".to_string(),
                        external_id: "amsreading".to_string(),
                    },
                },
                datapoints: DatapointsEnumType::NumericDatapoints(vec![DatapointDouble {
                    timestamp: time,
                    value: Some(value),
                    status: None,
                }]),
            }],
            &|id_or_instance| {
                id_or_instance
                    .iter()
                    .map(|v| match v {
                        IdentityOrInstance::Identity(Identity::ExternalId { external_id }) => {
                            AddDmOrTimeSeries::TimeSeries(Box::new(AddTimeSeries {
                                external_id: Some(external_id.to_string()),
                                ..Default::default()
                            }))
                        }
                        IdentityOrInstance::InstanceId { instance_id } => {
                            let mut timeseries = CogniteTimeseries::new(
                                instance_id.space.to_string(),
                                instance_id.external_id.to_string(),
                                Timeseries::new(false),
                            );
                            timeseries.properties.r#type = TimeSeriesType::Numeric;
                            AddDmOrTimeSeries::Cdm(Box::new(timeseries))
                        }
                        _ => panic!("Invalid identity received."),
                    })
                    .collect::<Vec<_>>()
                    .into_iter()
            },
        )
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    let client = cog::get_client();
    subscribe(client).await;
}
