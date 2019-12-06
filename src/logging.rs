use chrono;
use fern;

pub(crate) fn set_up_logging() {
    use fern::colors::{Color, ColoredLevelConfig};

    let colors_line = ColoredLevelConfig::new()
        .error(Color::BrightRed)
        .warn(Color::Yellow);

    let colors_level = colors_line.clone().info(Color::Green).debug(Color::Cyan);
    // here we set up our fern Dispatch
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        // set the default log level. to filter out verbose log messages from dependencies, set
        // this to Warn and overwrite the log level for your crate.
        .level(log::LevelFilter::Warn)
        .level_for("monitor_websites", log::LevelFilter::Trace)
        .chain(std::io::stdout())
        .apply()
        .expect("logging setup should be OK");
}
