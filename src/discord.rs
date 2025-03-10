pub trait ToDiscordMessage {
    fn message(&self) -> String;
}
