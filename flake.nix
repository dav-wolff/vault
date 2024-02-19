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
	};
	
	outputs = { self, nixpkgs, flake-utils, crane, fenix }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs {
					inherit system;
				};
				
				stylance = pkgs.callPackage ./stylance.nix {};
				
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
				craneLib = crane.lib.${system}.overrideToolchain fenixToolchain;
				
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
				
				makeVault = {dbFile ? null}:
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
						
						buildArgs = commonArgs // {
							VAULT_DB_FILE = dbFile;
						};
						
						bin = craneLib.buildPackage (buildArgs // {
							cargoArtifacts = cargoBinArtifacts;
							pname = "${pname}-bin";
							cargoExtraArgs = "--locked --bins --features=ssr";
						});
						
						wasm = craneLib.buildPackage (buildArgs // {
							cargoArtifacts = cargoWasmArtifacts;
							pname = "${pname}-wasm";
							nativeBuildInputs = with pkgs; [
								wasm-pack
								wasm-bindgen-cli
								binaryen
								nodePackages.uglify-js
							];
							
							buildPhaseCargoCommand = ''
								mkdir temp-home &&
								HOME=$(pwd)/temp-home
								CARGO_LOG=TRACE
								wasm-pack build --target web --release --no-typescript --no-pack -- --locked --features=hydrate
							'';
							
							installPhaseCommand = ''
								mkdir -p $out &&
								mv pkg/${pname}_bg.wasm $out/${pname}_bg.wasm &&
								uglifyjs pkg/${pname}.js -o $out/${pname}.js --verbose
							'';
							
							doCheck = false; # can't run tests for wasm build
						});
						
						package = pkgs.stdenv.mkDerivation {
							inherit pname version src;
							
							nativeBuildInputs = with pkgs; [
								makeWrapper
								stylance
								dart-sass
								lightningcss
							];
							
							buildPhase = ''
								stylance . --output-file vault.scss &&
								sass vault.scss vault.css
							'';
							
							installPhase = ''
								mkdir -p $out/bin &&
								cp ${bin}/bin/${pname} $out/bin/${pname} &&
								mv assets $out/site &&
								cp -r ${wasm} $out/site/pkg &&
								chmod +w $out/site/pkg &&
								lightningcss --minify vault.css --output-file $out/site/pkg/vault.css
							'';
							
							fixupPhase = ''
								wrapProgram $out/bin/${pname} \
									--set LEPTOS_OUTPUT_NAME ${pname} \
									--set LEPTOS_SITE_ROOT $out/site \
									--set LEPTOS_ENV PROD
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
						export VAULT_DB_FILE="dev_data/vault.db"
					'';
					
					packages = with pkgs; [
						fenixRustAnalyzer
						just
						cargo-leptos
						binaryen
						stylance
						dart-sass
					];
				};
			}
		);
}
