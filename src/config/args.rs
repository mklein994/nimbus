//#![cfg_attr(feature = "clippy", plugin(clippy))]
//#![cfg_attr(feature = "clippy", warn(clippy_pedantic))]
//#![cfg_attr(feature = "clippy", allow(missing_docs_in_private_items))]

use clap::{App, Arg};

const LOCATION_HELP: &str = "\
Latitude and longitude (in decimal degrees), separated by a colon.
Latitude: Positive is north, negative is south.
Longitude: Positive is east, negative is west.";

const UNITS_HELP: &str = "\
\u{2218} auto: automatically select units based on geographic location

\u{2218} ca: same as si, except that wind speed and wind gust are in kilometers per hour

\u{2218} uk2: same as si, except that nearest storm distance and visibility are in miles, and wind \
speed and wind gust are in miles per hour

\u{2218} us: imperial units

\u{2218} si: SI units";

pub fn build_cli() -> App<'static, 'static> {
    App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("config")
                .empty_values(false)
                .short("c")
                .long("config")
                .number_of_values(1)
                .help("Path to configuration")
                .long_help("Path to configuration. Follows the XDG Base Directory Specification."),
        )
        .arg(
            Arg::with_name("location")
                .short("l")
                .long("location")
                .value_names(&["LAT", "LON"])
                .value_delimiter(":")
                .require_delimiter(true)
                .help("Latitude and longitude (in decimal degrees), separated by a colon")
                .long_help(LOCATION_HELP),
        )
        .arg(
            Arg::with_name("language")
                .long("language")
                .short("g")
                .default_value("en")
                .help("Language to show results in.")
                .long_help(
                    "Supported values come from <https://darksky.net/dev/docs#request-parameters>.",
                ),
        )
        .arg(
            Arg::with_name("units")
                .long("units")
                .short("u")
                .possible_values(&["auto", "ca", "uk2", "us", "si"])
                .default_value("auto")
                .help("Unit display weather conditions will be shown in.")
                .long_help(UNITS_HELP),
        )
}
