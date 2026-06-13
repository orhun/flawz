use clap::Parser;

use crate::theme::BuiltinTheme;

/// Command line arguments.
#[derive(Debug, Default, Parser)]
#[clap(
    version,
    author = clap::crate_authors!("\n"),
    about,
    rename_all_env = "screaming-snake",
    help_template = "\
{before-help}{name} {version} - {about}
With ♥ by {author-with-newline}
{usage-heading}
  {usage}

{all-args}{after-help}
",
)]
pub struct Args {
    /// Feeds to sync. Accepts a year (`2024`), a year range (`2002:2024`),
    /// `recent` (last 8 days of new publications) or `modified` (last 8
    /// days of modifications). Multiple feeds can be given.
    #[arg(
        short,
        long,
        env,
        num_args(0..),
        default_values_t = ["2002:2025".to_string(), "recent".into(), "modified".into()]
    )]
    pub feeds: Vec<String>,

    /// Path to the SQLite database used to store the synced CVE data.
    #[arg(short, env, long)]
    pub db: Option<String>,

    /// NVD API key. With a key the rate limit is 50 requests / 30s
    /// (instead of 5 / 30s), making sync roughly 10× faster. Get one at
    /// <https://nvd.nist.gov/developers/request-an-api-key>.
    #[arg(short = 'k', long, env = "NVD_API_KEY")]
    pub api_key: Option<String>,

    /// Re-sync feeds that are already present in the cache.
    #[arg(short = 'u', long)]
    pub force_update: bool,

    /// Do not fetch feeds — read only what is already cached.
    #[arg(short, long)]
    pub offline: bool,

    /// Start with a search query.
    #[arg(short, env, long)]
    pub query: Option<String>,

    /// Set the theme.
    #[arg(short, long, value_enum, default_value = "dracula")]
    pub theme: BuiltinTheme,
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
