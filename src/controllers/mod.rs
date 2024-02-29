pub mod transaction;
pub mod wallet;

// 队列key名称
pub const QUEUE_KEY: &str = "muse:credits-bill:transaction_queue";
// 当前锁key名称
pub const LOCK_KEY: &str = "muse:credits-bill:transaction_key";
// 当前锁key过期时间
pub const LOCK_KEY_TTL: usize = 10;

// 当前交易信息key 防丢失
pub const LOCK_TRANSACTION_KEY: &str = "muse:credits-bill:transaction_key_lock";


