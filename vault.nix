{ lib
, craneLib
, stdenv
, wasm-pack
, wasm-bindgen-cli
, binaryen
, uglify-js
, makeWrapper
, stylance-cli
, cachebust
, dart-sass
, lightningcss
, port ? null
, authKey ? null
, dbFile ? null
, filesLocation ? null
}:

let
	nameVersion = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };
	pname = nameVersion.pname;
	version = nameVersion.version;
	
	src = with lib; cleanSourceWith {
		src = craneLib.path ./.;
		filter = path: type:
			(hasInfix "/assets/" path) ||
			(hasInfix "/style/" path) ||
			(hasInfix "/src/" path && (hasSuffix ".css" path || hasSuffix ".scss" path)) ||
			(craneLib.filterCargoSources path type)
		;
	};
	
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
		nativeBuildInputs = [
			wasm-pack
			wasm-bindgen-cli
			binaryen
			uglify-js
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
in stdenv.mkDerivation {
	inherit pname version src;
	
	passthru = {
		inherit wasm bin;
	};
	
	nativeBuildInputs = [
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
}
