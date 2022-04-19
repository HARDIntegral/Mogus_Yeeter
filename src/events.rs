use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::Message;
use serenity::model::channel::ChannelType;
use serenity::prelude::*;
use serenity::utils::MessageBuilder;
use  serenity::model::id::ChannelId;
use rand::Rng;
use std::env;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready", ready.user.name);
    }

    async fn message(&self, context: Context, msg: Message) {
        println!("{} attachments sent", msg.attachments.len());
    }
}