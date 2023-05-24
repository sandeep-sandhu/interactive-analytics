use interactive_analytics;

use interactive_analytics::logging::SimpleDebugLogger;
use interactive_analytics::logging::SimpleInfoLogger;

static DBG_LOGGER: SimpleDebugLogger = SimpleDebugLogger;
static LOGGER: SimpleInfoLogger = SimpleInfoLogger;

// initialise the debug logger
pub fn init_debug_logger() {
    log::set_logger(&DBG_LOGGER).expect("Could not set the debug logger!");
    log::set_max_level(log::LevelFilter::Debug);
}

pub fn init_logger() {
    log::set_logger(&LOGGER).expect("Could not set the logger!");
    log::set_max_level(log::LevelFilter::Debug);
}

// TODO: define useful functions for testing


#[test]
fn first_test() {
    println!("TODO: Implement a test");

    assert_eq(
        true,
        true,
        "First Test - checking whether true == true"
    )
}

