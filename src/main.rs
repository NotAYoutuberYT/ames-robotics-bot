use serenity::{
    async_trait,
    model::{
        application::interaction::Interaction,
        gateway::Ready,
        prelude::{command::Command, interaction::InteractionResponseType},
    },
    prelude::{Client, Context, EventHandler, GatewayIntents, TypeMapKey},
    Error,
};

// not included in repo, configure manually
mod bot_token;
use bot_token::BOT_TOKEN;

mod admin_config;
use admin_config::{admin_list::AdminRoleList, give_admin, list_config_roles, remove_admin};

// bot persistent data
struct AdminRoles;
impl TypeMapKey for AdminRoles {
    type Value = AdminRoleList;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // slash command handler
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // quick console log
            println!("{} invoked {}", command.user.name, command.data.name);

            // run the command and store the result
            let result: Result<(), Error> = match command.data.name.as_str() {
                "give-config-permissions" => give_admin::run(&command, &ctx).await,
                "remove-config-permissions" => remove_admin::run(&command, &ctx).await,
                "list-configuration-roles" => list_config_roles::run(&command, &ctx).await,
                _ => Err(Error::Other("Command not implemented")),
            };

            // if an error occured, log it
            if let Err(e) = result {
                command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content("Sorry, an error occured. Ask the person hosting the bot to check the logs!")
                            })
                    })
                    .await.unwrap_or_default();
                println!("Error when running command {}: {:?}", command.data.name, e)
            }
        }
    }

    // shard start handler
    async fn ready(&self, ctx: Context, ready: Ready) {
        // create the slash commands using
        let commands = Command::set_global_application_commands(&ctx.http, |command_builder| {
            command_builder
                .create_application_command(|command| give_admin::create(command))
                .create_application_command(|command| remove_admin::create(command))
                .create_application_command(|command| list_config_roles::create(command))
        })
        .await
        .expect("Error adding slash commands");

        // prints a message
        println!(
            "Currently enabled slash command:{}\n\n-------------------\n\tLog\n-------------------\n\nRunning bot {} (id {})",
            commands
                .into_iter()
                .fold("".to_owned(), |msg, command| msg + "\n   " + &command.name),
            ready.user.name,
            ready.user.id,
        );
    }
}

#[tokio::main]
async fn main() {
    // build client
    let mut client = Client::builder(BOT_TOKEN, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // configure bot data in a group to close the rwlock
    {
        // open data rwlock
        let mut data = client.data.write().await;

        // insert data
        data.insert::<AdminRoles>(AdminRoleList::new());
    }

    // start shard
    if let Err(e) = client.start_shards(1).await {
        println!("Error starting client: {:?}", e);
    }
}
