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
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs {
					inherit system;
				};
				
				cachebust = cache_bust.packages.${system}.cli;
				
				fenixPackage = fenix.packages.${system};
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
				fenixRustAnalyzer = fenixPackage.rust-analyzer;
				craneLib = (crane.mkLib pkgs).overrideToolchain fenixToolchain;
				
				nameVersion = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };
				pname = nameVersion.pname;
				version = nameVersion.version;
				
				src = with pkgs.lib; cleanSourceWith {
					src = craneLib.path ./.;
					filter = path: type:
						(hasInfix "/assets/" path) ||
						(hasInfix "/style/" path) ||
						(hasInfix "/src/" path && (hasSuffix ".css" path || hasSuffix ".scss" path)) ||
						(craneLib.filterCargoSources path type)
					;
				};
				
				makeVault = {port ? null, authKey ? null, dbFile ? null, filesLocation ? null}:
					let
						commonArgs = {
							inherit src;
						};
						
						cargoBinArtifacts = craneLib.buildDepsOnly (commonArgs // {
							pname = "${pname}-bin";
							cargoExtraArgs = "--locked --features=ssr";
						});
						
						cargoWasmArtifacts = craneLib.buildDepsOnly (commonArgs // {
							pname = "${pname}-wasm";
							cargoBuildCommand = "cargo build --profile wasm-release";
							cargoExtraArgs = "--locked --target=wasm32-unknown-unknown --features=hydrate";
						});
						
						bin = craneLib.buildPackage (commonArgs // {
							cargoArtifacts = cargoBinArtifacts;
							pname = "${pname}-bin";
							cargoExtraArgs = "--locked --bins --features=ssr";
							VAULT_PORT = port;
							VAULT_AUTH_KEY = authKey;
							VAULT_DB_FILE = dbFile;
							VAULT_FILES_LOCATION = filesLocation;
						});
						
						wasm = craneLib.buildPackage (commonArgs // {
							cargoArtifacts = cargoWasmArtifacts;
							pname = "${pname}-wasm";
							nativeBuildInputs = with pkgs; [
								wasm-pack
								wasm-bindgen-cli
								binaryen
								nodePackages.uglify-js
							];
							
							buildPhaseCargoCommand = ''
								mkdir temp-home
								HOME=$(pwd)/temp-home
								CARGO_LOG=TRACE
								wasm-pack build --target web --release --no-typescript --no-pack -- --locked --features=hydrate
							'';
							
							installPhaseCommand = ''
								mkdir -p $out
								mv pkg/${pname}_bg.wasm $out/${pname}_bg.wasm
								uglifyjs pkg/${pname}.js -o $out/${pname}.js --verbose
							'';
							
							doCheck = false; # can't run tests for wasm build
						});
						
						package = pkgs.stdenv.mkDerivation {
							inherit pname version src;
							
							nativeBuildInputs = with pkgs; [
								makeWrapper
								stylance-cli
								cachebust
								dart-sass
								lightningcss
							];
							
							buildPhase = ''
								stylance . --output-file vault.scss
								sass vault.scss vault.css
							'';
							
							installPhase = ''
								mkdir -p $out/bin
								cp ${bin}/bin/${pname} $out/bin/${pname}
								cachebust assets --out $out/site
								cp -r ${wasm} $out/site/pkg
								chmod +w $out/site/pkg
								lightningcss --minify vault.css --output-file $out/site/pkg/vault.css
								echo -n "wasm: " > $out/bin/hashes.txt
								cachebust $out/site/pkg --file vault_bg.wasm --print hash >> $out/bin/hashes.txt
								echo -n "js: " >> $out/bin/hashes.txt
								cachebust $out/site/pkg --file vault.js --print hash >> $out/bin/hashes.txt
								echo -n "css: " >> $out/bin/hashes.txt
								cachebust $out/site/pkg --file vault.css --print hash >> $out/bin/hashes.txt
							'';
							
							fixupPhase = ''
								wrapProgram $out/bin/${pname} \
									--set LEPTOS_OUTPUT_NAME ${pname} \
									--set LEPTOS_SITE_ROOT $out/site \
									--set LEPTOS_ENV PROD \
									--set LEPTOS_HASH_FILES true \
									--set LEPTOS_HASH_FILE_NAME hashes.txt
							'';
						};
					in {
						inherit wasm bin package;
					};
				
				defaultVault = makeVault {};
				
				wasm = defaultVault.wasm // {
					withAttrs = attrs: (makeVault attrs).wasm;
				};
				
				bin = defaultVault.bin // {
					withAttrs = attrs: (makeVault attrs).bin;
				};
				
				package = defaultVault.package // {
					withAttrs = attrs: (makeVault attrs).package;
				};
			in {
				packages.wasm = wasm;
				packages.bin = bin;
				packages.default = package;
				
				checks = {
					vault = package;
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
						fenixRustAnalyzer
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
