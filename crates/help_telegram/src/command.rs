use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Tanya apapun ke aku")]
    Ask,
    #[command(description = "Bantuan menggunakan bot ini")]
    Help,
}
