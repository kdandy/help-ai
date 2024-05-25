<h1 align="center">Help AI</h1>
<p align="center">A telegram bot that can talk and help you</p>

<p align="center">
You can try the bot here: <a href="#">Coming Soon</a>
</p>


<p align="center">
  <img align="center" src="https://github.com/kdandy/help-ai/assets/45036724/f6a0e236-f816-4402-91de-7ab467d43573">
</p>


## TODO

- [x] Telegram bot
- [ ] Run Bahasa Indonesia models
- [ ] Learn context
- [ ] Add Whatsapp port

## Run locally

1. Clone the repository
```sh
git clone https://github.com/kdandy/help-ai/
cd help-ai
```
2. Clone .env file
```
cp .env.example .env
```
3. Set up your values in .env 
```
TELOXIDE_TOKEN=
OPENROUTER_API=
RUST_LOG=trace
BOT_USERNAME=
```
5. Run the telegram bot
```
cargo run -p example_telegram
```

## License 
MIT
