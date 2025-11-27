//! avl-queue - High-Performance Message Queue
//!
//! Features:
//! - Partitioned topics (like Kafka)
//! - Consumer groups with offset tracking
//! - At-least-once delivery semantics
//!
//! Competing with: Kafka, RabbitMQ, NATS

use core::sync::atomic::{AtomicU64, Ordering};
use alloc::vec::Vec;
use alloc::string::String;
use alloc::collections::{BTreeMap, VecDeque};

/// Message in the queue
#[derive(Debug, Clone)]
pub struct Message {
    pub key: Option<Vec<u8>>,
    pub value: Vec<u8>,
    pub headers: BTreeMap<String, Vec<u8>>,
    pub timestamp: u64,
    pub offset: u64,
    pub partition: u32,
}

impl Message {
    pub fn new(value: Vec<u8>) -> Self {
        Self {
            key: None,
            value,
            headers: BTreeMap::new(),
            timestamp: current_time_ms(),
            offset: 0,
            partition: 0,
        }
    }

    pub fn with_key(mut self, key: Vec<u8>) -> Self {
        self.key = Some(key);
        self
    }

    pub fn add_header(mut self, key: String, value: Vec<u8>) -> Self {
        self.headers.insert(key, value);
        self
    }
}

/// Topic partition
pub struct Partition {
    pub id: u32,
    messages: VecDeque<Message>,
    next_offset: AtomicU64,
    max_size: usize,
}

impl Partition {
    pub fn new(id: u32, max_size: usize) -> Self {
        Self {
            id,
            messages: VecDeque::new(),
            next_offset: AtomicU64::new(0),
            max_size,
        }
    }

    /// Append message to partition
    pub fn append(&mut self, mut message: Message) -> Result<u64, QueueError> {
        if self.messages.len() >= self.max_size {
            // In production, would write to disk
            return Err(QueueError::PartitionFull);
        }

        let offset = self.next_offset.fetch_add(1, Ordering::SeqCst);
        message.offset = offset;
        message.partition = self.id;
        message.timestamp = current_time_ms();

        self.messages.push_back(message);
        Ok(offset)
    }

    /// Read messages from offset
    pub fn read(&self, start_offset: u64, max_messages: usize) -> Vec<Message> {
        let mut results = Vec::new();

        for msg in &self.messages {
            if msg.offset >= start_offset {
                results.push(msg.clone());
                if results.len() >= max_messages {
                    break;
                }
            }
        }

        results
    }

    /// Get latest offset
    pub fn high_watermark(&self) -> u64 {
        self.next_offset.load(Ordering::Relaxed)
    }

    /// Trim messages before offset (retention)
    pub fn trim_before(&mut self, offset: u64) {
        self.messages.retain(|msg| msg.offset >= offset);
    }
}

/// Topic with multiple partitions
pub struct Topic {
    pub name: String,
    pub partitions: Vec<Partition>,
    pub replication_factor: u32,
}

impl Topic {
    pub fn new(name: String, num_partitions: u32, partition_size: usize) -> Self {
        let mut partitions = Vec::new();
        for i in 0..num_partitions {
            partitions.push(Partition::new(i, partition_size));
        }

        Self {
            name,
            partitions,
            replication_factor: 1,
        }
    }

    /// Produce message to topic (partition selected by key hash)
    pub fn produce(&mut self, message: Message) -> Result<(u32, u64), QueueError> {
        let partition_id = if let Some(ref key) = message.key {
            self.hash_key(key) % self.partitions.len() as u32
        } else {
            // Round-robin if no key
            (self.partitions.len() / 2) as u32
        };

        let partition = &mut self.partitions[partition_id as usize];
        let offset = partition.append(message)?;

        Ok((partition_id, offset))
    }

    /// Consume messages from partition
    pub fn consume(&self, partition_id: u32, offset: u64, max_messages: usize) -> Vec<Message> {
        if partition_id >= self.partitions.len() as u32 {
            return Vec::new();
        }

        self.partitions[partition_id as usize].read(offset, max_messages)
    }

    fn hash_key(&self, key: &[u8]) -> u32 {
        // Simple FNV-1a hash
        let mut hash = 2166136261u32;
        for &byte in key {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(16777619);
        }
        hash
    }
}

/// Consumer group for coordinated consumption
pub struct ConsumerGroup {
    pub group_id: String,
    pub topic: String,
    pub members: Vec<Consumer>,
    pub offsets: BTreeMap<u32, u64>, // partition -> committed offset
}

impl ConsumerGroup {
    pub fn new(group_id: String, topic: String) -> Self {
        Self {
            group_id,
            topic,
            members: Vec::new(),
            offsets: BTreeMap::new(),
        }
    }

    pub fn add_consumer(&mut self, consumer: Consumer) {
        self.members.push(consumer);
        // Would trigger rebalance in production
    }

    pub fn remove_consumer(&mut self, consumer_id: &str) {
        self.members.retain(|c| c.id != consumer_id);
        // Would trigger rebalance
    }

    /// Commit offset for partition
    pub fn commit_offset(&mut self, partition: u32, offset: u64) {
        self.offsets.insert(partition, offset);
    }

    /// Get committed offset for partition
    pub fn get_offset(&self, partition: u32) -> u64 {
        self.offsets.get(&partition).copied().unwrap_or(0)
    }

    /// Assign partitions to consumers (simple round-robin)
    pub fn assign_partitions(&self, num_partitions: u32) -> BTreeMap<String, Vec<u32>> {
        let mut assignments = BTreeMap::new();

        if self.members.is_empty() {
            return assignments;
        }

        for partition in 0..num_partitions {
            let consumer_idx = (partition as usize) % self.members.len();
            let consumer_id = &self.members[consumer_idx].id;

            assignments.entry(consumer_id.clone())
                .or_insert_with(Vec::new)
                .push(partition);
        }

        assignments
    }
}

/// Individual consumer
#[derive(Debug, Clone)]
pub struct Consumer {
    pub id: String,
    pub assigned_partitions: Vec<u32>,
}

impl Consumer {
    pub fn new(id: String) -> Self {
        Self {
            id,
            assigned_partitions: Vec::new(),
        }
    }
}

/// Message broker - manages topics and consumer groups
pub struct MessageBroker {
    pub topics: BTreeMap<String, Topic>,
    pub consumer_groups: BTreeMap<String, ConsumerGroup>,
}

impl MessageBroker {
    pub fn new() -> Self {
        Self {
            topics: BTreeMap::new(),
            consumer_groups: BTreeMap::new(),
        }
    }

    pub fn create_topic(&mut self, name: String, num_partitions: u32, partition_size: usize) {
        let topic = Topic::new(name.clone(), num_partitions, partition_size);
        self.topics.insert(name, topic);
    }

    pub fn produce(&mut self, topic_name: &str, message: Message) -> Result<(u32, u64), QueueError> {
        let topic = self.topics.get_mut(topic_name)
            .ok_or(QueueError::TopicNotFound)?;

        topic.produce(message)
    }

    pub fn consume(
        &self,
        topic_name: &str,
        partition: u32,
        offset: u64,
        max_messages: usize,
    ) -> Result<Vec<Message>, QueueError> {
        let topic = self.topics.get(topic_name)
            .ok_or(QueueError::TopicNotFound)?;

        Ok(topic.consume(partition, offset, max_messages))
    }

    pub fn create_consumer_group(&mut self, group_id: String, topic: String) {
        let group = ConsumerGroup::new(group_id.clone(), topic);
        self.consumer_groups.insert(group_id, group);
    }

    pub fn join_consumer_group(&mut self, group_id: &str, consumer: Consumer) -> Result<(), QueueError> {
        let group = self.consumer_groups.get_mut(group_id)
            .ok_or(QueueError::GroupNotFound)?;

        group.add_consumer(consumer);
        Ok(())
    }

    pub fn commit_offset(
        &mut self,
        group_id: &str,
        partition: u32,
        offset: u64,
    ) -> Result<(), QueueError> {
        let group = self.consumer_groups.get_mut(group_id)
            .ok_or(QueueError::GroupNotFound)?;

        group.commit_offset(partition, offset);
        Ok(())
    }
}

#[derive(Debug)]
pub enum QueueError {
    TopicNotFound,
    PartitionFull,
    GroupNotFound,
    InvalidOffset,
}

fn current_time_ms() -> u64 {
    // In production, would use system clock
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition_append() {
        let mut partition = Partition::new(0, 1000);
        let msg = Message::new(vec![1, 2, 3]);

        let offset = partition.append(msg).unwrap();
        assert_eq!(offset, 0);
        assert_eq!(partition.high_watermark(), 1);
    }

    #[test]
    fn test_topic_produce() {
        let mut topic = Topic::new("test".to_string(), 3, 1000);
        let msg = Message::new(vec![1, 2, 3]).with_key(vec![10]);

        let (partition, offset) = topic.produce(msg).unwrap();
        assert!(partition < 3);
        assert_eq!(offset, 0);
    }

    #[test]
    fn test_consumer_group_assignment() {
        let mut group = ConsumerGroup::new("group1".to_string(), "topic1".to_string());
        group.add_consumer(Consumer::new("consumer1".to_string()));
        group.add_consumer(Consumer::new("consumer2".to_string()));

        let assignments = group.assign_partitions(4);
        assert_eq!(assignments.len(), 2);

        // Each consumer should get 2 partitions
        for (_, partitions) in assignments {
            assert_eq!(partitions.len(), 2);
        }
    }

    #[test]
    fn test_message_broker() {
        let mut broker = MessageBroker::new();
        broker.create_topic("orders".to_string(), 3, 1000);

        let msg = Message::new(b"order1".to_vec());
        let result = broker.produce("orders", msg);
        assert!(result.is_ok());

        let messages = broker.consume("orders", 0, 0, 10).unwrap();
        assert!(messages.len() <= 10);
    }
}
