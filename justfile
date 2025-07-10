default: clippy spell build test

s: spell
spell:
	typos --sort

spell-watch:
	watchexec "clear && typos --sort"

c: clippy
clippy:
	cargo clippy --all-targets -- -W clippy::pedantic

b: build
build: clippy 
	cargo build --release

t: test
test: build 
	cargo test

service: spell build
	@echo "Restarting service"
	sudo systemctl restart tg-bot.service
	@echo "Service restarted, status:"
	sudo systemctl status tg-bot.service
