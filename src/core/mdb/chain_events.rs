pub struct ChainEventData {
    pub current_progress: i64,
    pub max_progress: i64
}

pub struct ChainEvent {
    pub story_id: i64,
    pub data: ChainEventData
}