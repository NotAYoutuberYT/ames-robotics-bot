use serenity::{
    model::prelude::{
        interaction::application_command::ApplicationCommandInteraction, PartialGuild,
    },
    prelude::Context,
    Error,
};

pub async fn extract_partial_guild(
    command: &ApplicationCommandInteraction,
    ctx: &Context,
) -> Result<PartialGuild, Error> {
    if let Some(guild_id) = command.guild_id {
        return guild_id.to_partial_guild(&ctx.http).await;
    } else {
        // we failed to get a guild id
        return Err(Error::Other("failed to get a guild id"));
    }
}
