use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage, Message};

fn main() {
    println!("Creating consumer...");
    let mut consumer = Consumer::from_hosts(vec!["192.168.0.168:9092".to_owned()])
        .with_topic_partitions("quickstart-events".to_owned(), &[0])
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("".to_owned())
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();

    println!("Starting loop over messages...");
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for Message { value, .. } in ms.messages() {
                println!("{:?}", std::str::from_utf8(value));
            }
            match consumer.consume_messageset(ms) {
                Err(_) => println!("Failed to consume messages!"),
                Ok(_) => (),
            }
        }
        consumer.commit_consumed().unwrap();
    }
}
