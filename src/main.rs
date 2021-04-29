mod config;

use config::{TOKEN, PREFIX};

use chrono::FixedOffset;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::{channel::Message, gateway::{Ready, Activity}};
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

#[group]
#[commands(
    help,
    user,
    server,
    help,
)]
struct General;

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "help message").await?;

    Ok(())
}

#[command]
async fn user(ctx: &Context, msg: &Message) -> CommandResult {
    let user_name = msg.author.name.clone();

    let nick_name = match msg.author_nick(ctx).await {
        Some(nick) => nick,
        None => user_name.clone(),
    };

    let users_avatar = match msg.author.avatar_url() {
        Some(url) => url,
        None => String::from("https://discord.com/assets/322c936a8c8be1b803cd94861bdfa868.png"), // URL of default avatar
    };

    // UTC +09:00
    let create_date = msg.author.created_at().with_timezone(&FixedOffset::east(9*3600)).format("%Y-%m-%d %Z").to_string();

    if let Err(why) = msg.channel_id.send_message(ctx, |message| {

        message.embed(|embed| embed
            .colour(0x00ff00)
            .title(format!("{}'s information", nick_name))
            .description(format!("user name (not nick name): {}\nJoin to Discord at {}", user_name, create_date))
            .author(|f| f
                .name(nick_name)
                .icon_url(users_avatar)
            )
        )
    }).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
async fn server(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.send_message(ctx, |message | {
        message.embed(|embed | embed
            .title("This is a title")
            .description("This is a description")
            .image("attachment://ferris_eyes.png")
            .fields(vec![
                ("This is the first field", "This is a field body", true),
                ("This is the second field", "Both of these fields are inline", true),
            ])
            .field("This is the third field", "This is not an inline field", false)
            .footer(|footer| footer
                .text("This is a footer")
            )
        )
    }).await {
        println!("Error sending message: {:?}", why);
    }

    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
        ctx.set_activity(Activity::playing("労働")).await;
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix(PREFIX))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(TOKEN)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}