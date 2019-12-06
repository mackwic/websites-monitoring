use chrono;
use fern;
use log::{debug, error, info, trace, warn};
use reqwest;

fn main() {
    set_up_logging();
    info!("start of run");

    let websites = vec!["https://scalingo.com"];

    for site in websites {
        debug!("This site: {}", site);
        //  TODO let res = reqwest::get(site).await;

        // match res {
        //     Err(e) =>
        // }
        error!("error !")
    }

    info!("end of run");
}

fn set_up_logging() {
    use fern::colors::{Color, ColoredLevelConfig};

    let colors_line = ColoredLevelConfig::new()
        .error(Color::BrightRed)
        .warn(Color::Yellow);

    // configure colors for the name of the level.
    // since almost all of them are the some as the color for the whole line, we
    // just clone `colors_line` and overwrite our changes
    let colors_level = colors_line.clone().info(Color::Green);
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
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply()
        .expect("logging setup should be OK");
}
