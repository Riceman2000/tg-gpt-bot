# Systemd template

To use this template:
- Build the project with `cargo build --release`
- Configure the supplied template `tg-bot.service` file to your preferences:
  - Add your user name to the areas where `user` is 
  - Change directories to where you pulled the project. The working directory should point to the root of the repository.
- Run `sudo cp tg-bot.service /etc/systemd/system/tg-bot.service` to copy the file into the service file location.
- Then configure systemd to use the service file:
  - Reload the services: `sudo systemctl daemon-reload`
  - Enable starting on boot: `sudo systemctl enable tg-bot.service`
  - Start the service: `sudo systemctl start tg-bot.service`

If you'd like to reload the file with changes you've made, use the supplied refresh script.
