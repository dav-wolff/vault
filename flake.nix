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
	
	outputs = { self, nixpkgs, flake-utils, ... } @ inputs: let
		 makeCraneLib = pkgs: let
				fenixNative = pkgs.fenix.complete; # nightly
				fenixWasm = pkgs.fenix.targets.wasm32-unknown-unknown.latest; # nightly
				fenixToolchain = pkgs.fenix.combine [
					fenixNative.rustc
					fenixNative.rust-src
					fenixNative.cargo
					fenixNative.rust-docs
					fenixNative.clippy
					fenixWasm.rust-std
				];
			in (inputs.crane.mkLib pkgs).overrideToolchain fenixToolchain;
		in {
			overlays = {
				cachebust = final: prev: {
					cachebust = inputs.cache_bust.packages.${prev.system}.cli;
				};
				
				fenix = final: prev: {
					fenix = inputs.fenix.packages.${prev.system};
				};
				
				vault = final: prev: let
					craneLib = makeCraneLib final;
				in {
					vault-rs = prev.callPackage ./vault.nix {
						inherit craneLib;
					};
				};
				
				pinWasmBindgen = final: prev: {
					vault-rs = prev.vault-rs.override {
						inherit (nixpkgs.legacyPackages.${prev.system}) wasm-pack wasm-bindgen-cli;
					};
				};
				
				default = nixpkgs.lib.composeManyExtensions (with self.overlays; [
					cachebust
					fenix
					vault
					pinWasmBindgen
				]);
			};
		} // flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs {
					inherit system;
					overlays = [
						self.overlays.default
					];
				};
				craneLib = makeCraneLib pkgs;
			in {
				packages.wasm = pkgs.vault-rs.wasm;
				packages.bin = pkgs.vault-rs.bin;
				packages.default = pkgs.vault-rs;
				
				checks = {
					vault = self.packages.${system}.default;
				};
				
				devShells.default = craneLib.devShell {
					checks = self.checks.${system};
					
					shellHook = ''
						export VAULT_PORT="3000"
						export VAULT_AUTH_KEY="dev_data/auth.key"
						export VAULT_DB_FILE="dev_data/vault.db"
						export VAULT_FILES_LOCATION="dev_data/files"
						export CACHE_BUST_SKIP_HASHING="1"
					'';
					
					packages = with pkgs; [
						fenix.rust-analyzer
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
