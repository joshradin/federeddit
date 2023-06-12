use log4rs::append::console::ConsoleAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;
use crate::cli::LoggingArgs;

static LOG_ENCODING: &str = r#"{d(%Y-%m-%d %H:%M:%S%.3f)} [{h({l:7})}] \({({M}:{L}):>32.48}\) - {m}{n}"#;

/// initialize logging for a binary.
pub fn init_logging(logging_args: &LoggingArgs) {
    let filter = match logging_args.verbosity {
        0 => LevelFilter::Info,
        1 => LevelFilter::Debug,
        2 => LevelFilter::Trace,
        v => panic!("unsupported verbosity {}", v),
    };
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_ENCODING)))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(filter))
        .unwrap();

    log4rs::init_config(config).unwrap();
}