# Systemd template

To use this template:
- Build the project with `cargo build --release`
- Run `sudo cp tg-bot.service /etc/systemd/system/tg-bot.service` to copy the file into the service file location.
- Then configure systemd to use the service file:
  - Reload the services: `sudo systemctl daemon-reload`
  - Enable starting on boot: `sudo systemctl enable tg-bot.service`
  - Start the service: `sudo systemctl start tg-bot.service`

If you'd like to reload the file with changes you've made then use the supplied refresh script.
