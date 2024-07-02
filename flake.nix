{
	description = "Vault";
	
	inputs = {
		nixpkgs = {
			url = "github:nixos/nixpkgs/nixos-unstable";
		};
		
		flake-utils = {
			url = "github:numtide/flake-utils";
		};
		
		crane = {
			url = "github:ipetkov/crane";
			inputs.nixpkgs.follows = "nixpkgs";
		};
		
		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};
		
		cache_bust = {
			url = "github:dav-wolff/cache_bust";
			inputs.nixpkgs.follows = "nixpkgs";
			inputs.fenix.follows = "fenix";
		};
	};
	
	outputs = { self, nixpkgs, flake-utils, crane, fenix, cache_bust }:
		{
			overlays = {
				cachebust = final: prev: {
					cachebust = cache_bust.packages.${prev.system}.cli;
				};
				
				craneLib = final: prev: let
					fenixPackage = fenix.packages.${prev.system};
					fenixNative = fenixPackage.complete; # nightly
					fenixWasm = fenixPackage.targets.wasm32-unknown-unknown.latest;
					fenixToolchain = fenixPackage.combine [
						fenixNative.rustc
						fenixNative.rust-src
						fenixNative.cargo
						fenixNative.rust-docs
						fenixNative.clippy
						fenixWasm.rust-std
					];
					craneLib = (crane.mkLib final).overrideToolchain fenixToolchain;
				in {
					inherit craneLib;
				};
				
				vault = final: prev: {
					vault-rs = prev.callPackage ./vault.nix {};
				};
				
				default = with self.overlays; nixpkgs.lib.composeManyExtensions [cachebust craneLib vault];
			};
		} // flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs {
					inherit system;
					overlays = [
						self.overlays.default
					];
				};
			in {
				packages.wasm = pkgs.vault-rs.wasm;
				packages.bin = pkgs.vault-rs.bin;
				packages.default = pkgs.vault-rs;
				
				checks = {
					vault = self.packages.${system}.default;
				};
				
				devShells.default = pkgs.craneLib.devShell {
					checks = self.checks.${system};
					
					shellHook = ''
						export VAULT_PORT="3000"
						export VAULT_AUTH_KEY="dev_data/auth.key"
						export VAULT_DB_FILE="dev_data/vault.db"
						export VAULT_FILES_LOCATION="dev_data/files"
						export CACHE_BUST_SKIP_HASHING="1"
					'';
					
					packages = with pkgs; [
						fenix.packages.${system}.rust-analyzer
						just
						cargo-leptos
						binaryen
						stylance-cli
						dart-sass
					];
				};
			}
		);
}
