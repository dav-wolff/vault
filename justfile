watch:
	cargo leptos watch &\
	stylance --watch $(dirname $(cargo locate-project --message-format plain))
