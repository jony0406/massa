use massa_models::config::constants::{GENESIS_TIMESTAMP, T0, THREAD_COUNT, GENESIS_KEY, MAX_GAS_PER_BLOCK, DELTA_F0, OPERATION_VALIDITY_PERIODS, PERIODS_PER_CYCLE, ENDORSEMENT_COUNT, CHANNEL_SIZE};
use massa_time::MassaTime;

use crate::GraphConfig;

impl Default for GraphConfig {
    fn default() -> Self {
        Self {
            clock_compensation_millis: 0,
            genesis_timestamp: *GENESIS_TIMESTAMP,
            t0: T0,
            thread_count: THREAD_COUNT,
            genesis_key: GENESIS_KEY.clone(),
            max_discarded_blocks: 10000,
            future_block_processing_max_periods: 100,
            max_future_processing_blocks: 100,
            max_dependency_blocks: 2048,
            max_send_wait: MassaTime::from_millis(100),
            block_db_prune_interval: MassaTime::from_millis(5000),
            max_item_return_count: 100,
            max_gas_per_block: MAX_GAS_PER_BLOCK,
            delta_f0: DELTA_F0,
            operation_validity_periods: OPERATION_VALIDITY_PERIODS,
            periods_per_cycle: PERIODS_PER_CYCLE,
            force_keep_final_periods: 20,
            endorsement_count: ENDORSEMENT_COUNT,
            end_timestamp: None,
            stats_timespan: MassaTime::from_millis(60000),
            channel_size: CHANNEL_SIZE,
        }
    }
}
