{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    utils.url = "github:numtide/flake-utils";
    worker-build.url = "github:lemmih/nix-flakes?dir=worker-build";
    worker-build.inputs.nixpkgs.follows = "nixpkgs";
    wrangler.url = "github:ryand56/wrangler";
    wrangler.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    alejandra.url = "github:kamadorueda/alejandra/3.1.0";
    alejandra.inputs.nixpkgs.follows = "nixpkgs";
    crane.url = "github:ipetkov/crane";
    e2e = {
      url = "path:./e2e";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    utils,
    worker-build,
    wrangler,
    rust-overlay,
    alejandra,
    crane,
    e2e,
  }:
    utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };
        pinned-wasm-bindgen-cli = pkgs.wasm-bindgen-cli.override {
          version = "0.2.100";
          hash = "sha256-3RJzK7mkYFrs7C/WkhW9Rr4LdP5ofb2FdYGz1P7Uxog=";
          cargoHash = "sha256-tD0OY2PounRqsRiFh8Js5nyknQ809ZcHMvCOLrvYHRE=";
        };
        worker-build-bin = worker-build.packages.${system}.default;
        wrangler-bin = wrangler.packages.${system}.default;

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        # Initialize crane with our custom toolchain
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        hiit-client-deps = craneLib.buildDepsOnly {
          src = ./.;
          cargoExtraArgs = "--target wasm32-unknown-unknown --features hydrate --no-default-features";
          doCheck = false;
        };

        # Create a derivation for building the client-side Wasm using crane
        hiit-client = craneLib.buildPackage {
          src = ./.;
          cargoArtifacts = hiit-client-deps;
          buildPhaseCargoCommand = "HOME=$PWD/tmp wasm-pack build --out-dir pkg --mode no-install --no-typescript --release --target web --out-name client --features hydrate --no-default-features";
          doNotPostBuildInstallCargoBinaries = true;
          installPhaseCommand = ''
            mkdir -p $out/pkg
            cp -r pkg/* $out/pkg/
          '';
          doCheck = false;

          nativeBuildInputs = with pkgs; [
            wasm-pack
            pinned-wasm-bindgen-cli
            binaryen
          ];
        };

        hiit-server-deps = craneLib.buildDepsOnly {
          src = ./.;
          cargoExtraArgs = "--target wasm32-unknown-unknown --features ssr --no-default-features";
          doCheck = false;
        };

        # Create a derivation for building the server-side Wasm using crane
        hiit-server = craneLib.buildPackage {
          src = ./.;
          cargoArtifacts = hiit-server-deps;
          buildPhaseCargoCommand = "HOME=$PWD/tmp worker-build --release --features ssr --no-default-features";
          doNotPostBuildInstallCargoBinaries = true;
          doCheck = false;
          installPhaseCommand = ''
            mkdir -p $out/build
            cp -r build/* $out/build/
          '';

          nativeBuildInputs = with pkgs; [
            worker-build-bin
            pinned-wasm-bindgen-cli
            binaryen
            esbuild
          ];
        };

        # Create the main hiit derivation that combines everything
        hiit = pkgs.stdenv.mkDerivation {
          name = "hiit";
          src = ./.;

          nativeBuildInputs = with pkgs; [
            tailwindcss
          ];

          buildPhase = ''
            # Generate CSS
            tailwindcss --minify -i $src/style/tailwind.css -o style.css
          '';

          installPhase = ''
            # Create the output directory structure
            mkdir -p $out/assets

            # Copy static files
            cp -r $src/public/* $out/assets/

            # Copy generated CSS
            cp style.css $out/assets/style.css

            # Copy wasm build outputs from other derivations
            cp -r ${hiit-client}/* $out/assets/
            cp -r ${hiit-server}/build $out/
          '';
        };

        # Create a development environment with a script to run wrangler
        hiit-dev = pkgs.writeScriptBin "hiit-dev" ''
          #!${pkgs.bash}/bin/bash

          # Create a temporary directory for the development environment
          WORK_DIR=$(mktemp -d)

          # Link the necessary directories
          ln -s ${hiit}/assets $WORK_DIR/assets
          ln -s ${hiit-server}/build $WORK_DIR/build

          # Copy the wrangler configuration
          cp ${./wrangler.toml} $WORK_DIR/wrangler.toml

          # Change to the work directory
          cd $WORK_DIR

          # Run wrangler in development mode
          exec ${wrangler-bin}/bin/wrangler dev --env prebuilt --live-reload false
        '';

        e2e-test = pkgs.writeShellScriptBin "e2e-test" ''
          # Start the web service
          ${hiit-dev}/bin/hiit-dev &
          WEB_PID=$!

          # Geckodriver is quite verbose, so we redirect the output to /dev/null
          # If you want to see the output, remove the redirection
          ${pkgs.geckodriver}/bin/geckodriver --port 4444 > /dev/null 2>&1 &
          GECKO_PID=$!

          # Run the tests
          ${self.packages.${system}.e2e}/bin/e2e
          TEST_EXIT=$?

          # Clean up
          kill $WEB_PID
          kill $GECKO_PID
          exit $TEST_EXIT
        '';

        # Create a deployment script for Cloudflare
        hiit-deploy = pkgs.writeScriptBin "hiit-deploy" ''
          #!${pkgs.bash}/bin/bash

          # Create a temporary directory for the deployment
          WORK_DIR=$(mktemp -d)
          echo "Created temporary directory: $WORK_DIR"

          # Copy the wrangler configuration
          cp ${./wrangler.toml} $WORK_DIR/wrangler.toml
          echo "Copied wrangler.toml to temporary directory"

          # Link the hiit derivation output to 'result'
          ln -s ${hiit} $WORK_DIR/result
          echo "Linked hiit build to result"

          # Change to the work directory
          cd $WORK_DIR
          echo "Changed to temporary directory"

          # Run wrangler to deploy
          echo "Deploying to Cloudflare..."
          ${wrangler-bin}/bin/wrangler deploy --env prebuilt
        '';
      in {
        packages = {
          inherit hiit hiit-client hiit-server;
          e2e = e2e.packages.${system}.default;
          default = hiit;
        };

        # Add the development app
        apps.default = {
          type = "app";
          program = "${hiit-dev}/bin/hiit-dev";
        };

        # Add the deployment app
        apps.deploy = {
          type = "app";
          program = "${hiit-deploy}/bin/hiit-deploy";
        };

        apps.e2e-test = {
          type = "app";
          program = "${e2e-test}/bin/e2e-test";
        };

        formatter = alejandra.packages.${system}.default;
      }
    );
}
