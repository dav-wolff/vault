default:
	@just --list --unsorted

watch:
	cargo leptos watch &\
	stylance --watch $(dirname $(cargo locate-project --message-format plain))

serve:
	@cd $(dirname $(cargo locate-project --message-format plain))
	nix build
	result/bin/vault
