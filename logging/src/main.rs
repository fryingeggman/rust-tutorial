use env_logger::{Builder, Env, Logger, Target};
use log::{debug, error, info, trace, warn, LevelFilter};

fn main() {
    // env_logger应该尽早初始化
    // env_logger::init();

    let env = Env::new()
        .filter_or("MY_LOG", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    // let mut builder = Builder::new();
    // builder.filter_level(LevelFilter::Info);
    // builder.init();

    // let mut builder = Builder::from_default_env();
    // builder
    //     .filter_level(LevelFilter::Error)
    //     .filter_level(LevelFilter::Warn)
    //     .filter_level(LevelFilter::Info)
    //     .filter_level(LevelFilter::Debug)
    //     .target(Target::Stdout);
    //builder.init();

    error!("error");
    warn!("warning");
    info!("info");
    debug!("debugging");
    trace!("trace");
}
