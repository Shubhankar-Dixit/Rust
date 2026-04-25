use clap::{Parser, ValueEnum};
use color_eyre::eyre::Result;
use serde::Deserialize;

#[derive(Deserialize)]
struct Story {
    title: String,
    url: Option<String>,
    score: u32,
    by: String,
    descendants: Option<u32>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum StoryKind {
    Top,
    New,
    Best,
    Ask,
    Show,
}

impl StoryKind {
    fn endpoint(self) -> &'static str {
        match self {
            Self::Top => "topstories",
            Self::New => "newstories",
            Self::Best => "beststories",
            Self::Ask => "askstories",
            Self::Show => "showstories",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Top => "Top",
            Self::New => "New",
            Self::Best => "Best",
            Self::Ask => "Ask",
            Self::Show => "Show",
        }
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "hn_cli",
    about = "Fetches Hacker News stories by category",
    version
)]
struct Cli {
    #[arg(value_enum, default_value_t = StoryKind::Top)]
    kind: StoryKind,

    #[arg(default_value_t = 10, value_parser = clap::value_parser!(u8).range(1..=100))]
    count: u8,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    println!("🔶 {} {} Hacker News Stories\n", cli.kind.label(), cli.count);

    let client = reqwest::blocking::Client::new();
    let stories_url = format!(
        "https://hacker-news.firebaseio.com/v0/{}.json",
        cli.kind.endpoint()
    );

    let top_ids: Vec<u64> = client
        .get(stories_url)
        .send()?
        .error_for_status()?
        .json()?;

    for (i, id) in top_ids.iter().take(usize::from(cli.count)).enumerate() {
        let url = format!("https://hacker-news.firebaseio.com/v0/item/{id}.json");

        let story: Story = client
            .get(&url)
            .send()?
            .error_for_status()?
            .json()?;

        let link = story.url.as_deref().unwrap_or("(no URL)");
        let comments = story.descendants.unwrap_or(0);
        println!(
            "{}. {} ({} points, {} comments by {})",
            i + 1,
            story.title,
            story.score,
            comments,
            story.by
        );
        println!("   {}\n", link);
    }

    Ok(())
}