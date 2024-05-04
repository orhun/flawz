use clap::Parser;

use crate::{app::AppResult, error::Error, theme::BuiltinTheme};

/// Command line arguments.
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
        default_value = "https://nvd.nist.gov/feeds/json/cve/1.1/"
    )]
    pub url: String,

    /// List of feeds that are going to be synced.
    #[arg(
        short,
        long,
        env,
        num_args(0..),
        default_values_t = ["2002:2024".to_string(), "recent".into(), "modified".into()]
    )]
    pub feeds: Vec<String>,

    /// Path to the SQLite database used to store the synced CVE data.
    #[arg(short, env, long)]
    pub db: Option<String>,

    /// Always fetch feeds.
    #[arg(long)]
    pub force_update: bool,

    /// Do not fetch feeds.
    #[arg(short, long)]
    pub offline: bool,

    /// Start with the search query.
    #[arg(short, env, long)]
    pub query: Option<String>,

    /// Sets the theme.
    #[arg(short, long, value_enum, default_value = "dracula")]
    pub theme: BuiltinTheme,
}

impl Args {
    /// Parses and returns the feeds.
    pub fn feeds(&self) -> AppResult<Vec<String>> {
        self.feeds
            .iter()
            .try_fold::<Vec<_>, _, AppResult<_>>(vec![], |mut acc, v| {
                if v.contains(':') {
                    let mut parts = v.split(':');
                    let start = parts
                        .next()
                        .and_then(|v| v.parse::<usize>().ok())
                        .ok_or_else(|| Error::RangeArgsError)?;
                    let end = parts
                        .next()
                        .and_then(|v| v.parse::<usize>().ok())
                        .ok_or_else(|| Error::RangeArgsError)?;
                    acc.extend(
                        (start..=end)
                            .map(|v| v.to_string())
                            .collect::<Vec<String>>(),
                    );
                } else {
                    acc.push(v.clone())
                }
                Ok(acc)
            })
    }
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
