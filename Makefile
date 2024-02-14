# Deploy command pulls most recent changes and runs the script
deploy:
	git pull origin main -f
	cargo run
