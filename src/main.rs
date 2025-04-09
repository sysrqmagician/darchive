use std::{fs::File, io::BufWriter, time::Duration};

use askama::Template;
use clap::Parser;
use color_eyre::{Result, eyre::OptionExt};
use serde::{Deserialize, Serialize};
use serenity::{
    Client,
    all::{ChannelId, GatewayIntents, MessagePagination},
};

#[derive(Parser)]
struct Args {
    discord_token: String,
    channel_id: u64,
}

#[derive(Serialize, Deserialize)]
struct ArchiveMessage {
    author_id: u64,
    author_username: String,
    author_nickname: String,
    content: String,
    timestamp: String,
    linked_assets: Vec<ArchiveAsset>,
}

#[derive(Serialize, Deserialize)]
struct ArchiveAsset {
    filename: String,
    path: String,
}

#[derive(Serialize, Deserialize)]
struct ArchiveChannel {
    name: String,
    id: u64,
    description: String,
    messages: Vec<ArchiveMessage>,
}

#[derive(Template)]
#[template(path = "chat.html")]
struct ChatTemplate {
    channel: ArchiveChannel,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.enable_steady_tick(Duration::from_millis(100));
    spinner.set_message("Creating client...");

    let client = Client::builder(
        args.discord_token,
        GatewayIntents::MESSAGE_CONTENT | GatewayIntents::GUILD_MESSAGES,
    )
    .await?;

    spinner.set_message("Getting channel details...");
    let channel_id = ChannelId::new(args.channel_id);
    let channel = client.http.get_channel(channel_id).await?;
    let guild_channel = channel
        .guild()
        .ok_or_eyre("Channel has to be a guild channel")?;

    spinner.set_message("Creating archive directories...");
    std::fs::create_dir(format!("./{channel_id}"))?;
    std::fs::create_dir(format!("./{channel_id}/assets"))?;

    let mut archive_channel = ArchiveChannel {
        id: channel_id.into(),
        description: guild_channel.topic.unwrap_or("".into()),
        name: guild_channel.name,
        messages: Vec::new(),
    };

    let mut request_count = 1;
    spinner.set_message(format!("Getting messages (page {request_count})..."));

    let mut messages = client
        .http
        .get_messages(ChannelId::new(args.channel_id), None, Some(100))
        .await?;

    if messages.len() == 100 {
        loop {
            request_count += 1;
            spinner.set_message(format!("Getting messages (page {request_count})..."));

            let last = messages.last().ok_or_eyre("No messages retrieved")?;
            let mut new_messages = client
                .http
                .get_messages(
                    channel_id,
                    Some(MessagePagination::Before(last.id)),
                    Some(100),
                )
                .await?;
            let returned_count = new_messages.len();

            messages.append(&mut new_messages);

            if returned_count != 100 {
                break;
            }
        }
    }

    let mut message_archive: Vec<ArchiveMessage> = Vec::new();
    for message in messages.iter().rev() {
        let mut converted = ArchiveMessage {
            author_id: message.author.id.into(),
            author_username: message.author.name.clone(),
            author_nickname: message
                .author
                .nick_in(client.http.clone(), guild_channel.guild_id)
                .await
                .unwrap_or(
                    message
                        .author
                        .global_name
                        .clone()
                        .unwrap_or(message.author.name.clone()),
                ),

            content: message.content.clone(),
            timestamp: message.timestamp.to_rfc2822(),
            linked_assets: Vec::new(),
        };

        for attachment in &message.attachments {
            spinner.set_message(format!(
                "Downloading {} ({})...",
                attachment.filename, attachment.id
            ));
            let path = format!(
                "{}/assets/{}.{}",
                channel_id,
                attachment.id,
                attachment
                    .filename
                    .split(".")
                    .last()
                    .ok_or_eyre("Failed to split attachment filename")?
            );
            std::fs::write(path.clone(), attachment.download().await?)?;
            converted.linked_assets.push(ArchiveAsset {
                path: path
                    .split_once("/")
                    .ok_or_eyre("Failed to split path for relative asset link")?
                    .1
                    .into(),
                filename: attachment.filename.clone(),
            });
        }

        message_archive.push(converted);
    }

    archive_channel.messages = message_archive;

    spinner.set_message("Writing JSON...");
    serde_json::to_writer_pretty::<File, ArchiveChannel>(
        File::create(format!("{}/data.json", channel_id))?,
        &archive_channel,
    )?;

    spinner.set_message("Writing HTML...");
    let mut html_path = BufWriter::new(File::create(format!("{}/index.html", channel_id))?);
    ChatTemplate {
        channel: archive_channel,
    }
    .write_into(&mut html_path)?;

    spinner.finish_with_message("Done!");

    Ok(())
}
