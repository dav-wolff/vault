default:
	@just --list --unsorted

watch:
	cargo leptos watch &\
	stylance --watch .

serve:
	nix build
	result/bin/vault

clean:
	rm result
	rm -r dev_data
	rm -r target

clean-data:
	rm -r dev_data
