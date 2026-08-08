use cognite::models::instances::{CogniteDescribable, InstanceId, Timeseries};
use cognite::time_series::{AddDatapoints, AddTimeSeries, DatapointDouble, DatapointsEnumType};
use cognite::{CogniteClient, IdentityOrInstance};
use config::{CogniteConfig, MqttConfig};
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use std::time::Duration;
mod cog;
mod config;
mod decode;

async fn subscribe(
    cognite_client: CogniteClient,
    mqtt_config: &MqttConfig,
    cognite_config: &CogniteConfig,
) {
    let mut mqttoptions = MqttOptions::new(
        "rumqtt-async",
        mqtt_config.broker_host.clone(),
        mqtt_config.broker_port,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client
        .subscribe(mqtt_config.topic.clone(), QoS::AtMostOnce)
        .await
        .unwrap();
    loop {
        let notification = eventloop.poll().await.unwrap();
        if let Event::Incoming(msg) = notification
            && let Packet::Publish(msg) = msg
            && let Ok(value) = decode::decode(msg.payload.to_vec())
        {
            insert_datapoints(
                cognite_client.clone(),
                cognite_config,
                chrono::Utc::now().timestamp_millis(),
                value as f64,
            )
            .await;
        }
    }
}

async fn insert_datapoints(
    client: CogniteClient,
    cognite_config: &CogniteConfig,
    time: i64,
    value: f64,
) {
    use cognite::Identity;
    use cognite::models::instances::{CogniteTimeseries, TimeSeriesType};
    use cognite::time_series::AddDmOrTimeSeries;
    client
        .time_series
        .insert_datapoints_create_missing(
            vec![AddDatapoints {
                id: IdentityOrInstance::InstanceId {
                    instance_id: InstanceId {
                        space: cognite_config.timeseries_space.clone(),
                        external_id: cognite_config.timeseries_external_id.clone(),
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
                                // Timeseries::new(false),
                                Timeseries {
                                    description: CogniteDescribable {
                                        name: Some("PowerReadingsG25".to_string()),
                                        description: Some("Power readings G25".to_string()),
                                        tags: None,
                                        aliases: None,
                                    },
                                    is_step: false,
                                    ..Default::default()
                                },
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
    let config = config::load();
    let client = cog::get_client(&config.cognite);
    subscribe(client, &config.mqtt, &config.cognite).await;
}
