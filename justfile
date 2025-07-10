default: clippy spell build test

s: spell
spell:
	typos --sort

spell-watch:
	watchexec "clear && typos --sort"

c: clippy
clippy: spell
	cargo clippy --all-targets -- -W clippy::pedantic

b: build
build: clippy 
	cargo build --release

t: test
test:  
	cargo test

service: spell test build 
	@echo "Restarting service"
	sudo systemctl restart tg-bot.service
	@echo "Service restarted, status:"
	sudo systemctl status tg-bot.service
