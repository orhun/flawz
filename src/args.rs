use clap::Parser;

#[derive(Debug, Default, Parser)]
#[clap(
    version,
    author = clap::crate_authors!("\n"),
    about,
    rename_all_env = "screaming-snake",
    help_template = "\
{before-help}-=[ {name} {version} ]=-\n
{about-with-newline}Written by {author-with-newline}
{usage-heading}
  {usage}

{all-args}{after-help}
",
)]
pub struct Args {
    /// A URL where NIST CVE 1.1 feeds can be found.
    #[arg(
        short,
        long,
        env,
        default_value = "https://nvd.nist.gov/feeds/json/cve/1.1"
    )]
    pub url: String,

    /// List of feeds that are going to be synced.
    #[arg(
        short,
        long,
        env,
        num_args(0..),
        default_values_t = ["2024".to_string(), "recent".into(), "modified".into()]
    )]
    pub feeds: Vec<String>,

    /// Path to the SQLite database used to store the synced CVE data.
    #[arg(short, env, long)]
    pub db: Option<String>,

    /// Always fetch feeds.
    #[arg(long)]
    pub force_update: bool,

    /// Do not fetch feeds.
    #[arg(long)]
    pub offline: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    #[test]
    fn test_args() {
        Args::command().debug_assert();
    }
}
