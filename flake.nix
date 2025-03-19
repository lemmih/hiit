{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    utils.url = "github:numtide/flake-utils";
    worker-build.url = "github:lemmih/nix-flakes?dir=worker-build";
    worker-build.inputs.nixpkgs.follows = "nixpkgs";
    wrangler.url = "github:ryand56/wrangler/v4";
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
    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
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
    advisory-db,
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

        # Common source filter for all Rust builds
        # This filters out files not needed for the Rust compilation
        src =
          craneLib.cleanCargoSource (craneLib.path ./.)
          // {
            # Add additional Rust source files that might not be included by default
            # For example, if you have files outside of src/ that are needed:
            extraSrcGlobs = [];
          };

        hiit-client-deps = craneLib.buildDepsOnly {
          inherit src;
          cargoExtraArgs = "--target wasm32-unknown-unknown --features hydrate --no-default-features";
          doCheck = false;
        };

        # Function to create client-side Wasm builds with configurable options
        makeHiitClientBuild = {
          name,
          optimized ? true,
        }:
          craneLib.buildPackage {
            inherit src;
            cargoArtifacts = hiit-client-deps;
            buildPhaseCargoCommand = "HOME=$PWD/tmp wasm-pack build --out-dir pkg --mode no-install ${
              if optimized
              then "--release"
              else "--no-opt"
            } --no-typescript --target web --out-name client --features hydrate --no-default-features";
            doNotPostBuildInstallCargoBinaries = true;
            installPhaseCommand = ''
              mkdir -p $out/pkg
              cp -r pkg/* $out/pkg/
            '';
            doCheck = false;

            nativeBuildInputs = with pkgs;
              [
                wasm-pack
                pinned-wasm-bindgen-cli
              ]
              ++ (
                if optimized
                then [binaryen]
                else []
              );
          };

        # Create optimized and development client builds
        hiit-client = makeHiitClientBuild {
          name = "hiit-client";
          optimized = true;
        };
        hiit-client-dev = makeHiitClientBuild {
          name = "hiit-client-dev";
          optimized = false;
        };

        hiit-server-deps = craneLib.buildDepsOnly {
          inherit src;
          cargoExtraArgs = "--target wasm32-unknown-unknown --features ssr --no-default-features";
          doCheck = false;
        };

        # Function to create server-side Wasm builds with configurable options
        makeHiitServerBuild = {
          name,
          optimized ? true,
        }:
          craneLib.buildPackage {
            inherit src;
            cargoArtifacts = hiit-server-deps;
            buildPhaseCargoCommand = "HOME=$PWD/tmp worker-build ${
              if optimized
              then "--release"
              else "--no-opt"
            } --features ssr --no-default-features";
            doNotPostBuildInstallCargoBinaries = true;
            doCheck = false;
            installPhaseCommand = ''
              mkdir -p $out/build
              cp -r build/* $out/build/
            '';

            nativeBuildInputs = with pkgs;
              [
                worker-build-bin
                pinned-wasm-bindgen-cli
                esbuild
              ]
              ++ (
                if optimized
                then [binaryen]
                else []
              );
          };

        # Create optimized and development server builds
        hiit-server = makeHiitServerBuild {
          name = "hiit-server";
          optimized = true;
        };
        hiit-server-dev = makeHiitServerBuild {
          name = "hiit-server-dev";
          optimized = false;
        };

        # For the main derivation, we need a different source set that includes non-Rust files
        mainSrc = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = path: type:
            (pkgs.lib.hasPrefix "${toString ./public}" path)
            || (pkgs.lib.hasPrefix "${toString ./style}" path)
            || (pkgs.lib.hasPrefix "${toString ./src}" path);
        };

        # Function to create a hiit derivation to reduce duplication
        makeHiitDerivation = {
          name,
          clientBuild,
          serverBuild,
        }:
          pkgs.stdenv.mkDerivation {
            inherit name;
            src = mainSrc;

            nativeBuildInputs = with pkgs; [
              tailwindcss
            ];

            buildPhase = ''
              # Generate CSS
              tailwindcss --content "$src/**" -i ./style/tailwind.css -o style.css
            '';

            installPhase = ''
              # Create the output directory structure
              mkdir -p $out/assets

              # Copy static files
              cp -r $src/public/* $out/assets/

              # Copy generated CSS
              cp style.css $out/assets/style.css

              # Copy wasm build outputs from other derivations
              cp -r ${clientBuild}/* $out/assets/
              cp -r ${serverBuild}/build $out/
            '';
          };

        # Create production and development builds using the function
        hiit = makeHiitDerivation {
          name = "hiit";
          clientBuild = hiit-client;
          serverBuild = hiit-server;
        };

        hiit-dev = makeHiitDerivation {
          name = "hiit-dev";
          clientBuild = hiit-client-dev;
          serverBuild = hiit-server-dev;
        };

        # Create a function to setup wrangler environment
        makeWranglerScript = {
          name,
          wranglerArgs,
          verbose ? false,
        }:
          pkgs.writeScriptBin name ''
            #!${pkgs.bash}/bin/bash

            # Create a temporary directory for the environment
            WORK_DIR=$(mktemp -d)
            ${
              if verbose
              then "echo \"Created temporary directory: $WORK_DIR\""
              else ""
            }

            # Copy the wrangler configuration
            cp ${./wrangler.toml} $WORK_DIR/wrangler.toml
            ${
              if verbose
              then "echo \"Copied wrangler.toml to temporary directory\""
              else ""
            }

            # Setup the environment
            ln -s ${hiit} $WORK_DIR/result

            # Change to the work directory
            cd $WORK_DIR
            ${
              if verbose
              then "echo \"Changed to temporary directory\""
              else ""
            }

            # Run wrangler with the provided arguments
            ${
              if verbose
              then "echo \"Running wrangler with args: ${wranglerArgs}...\""
              else ""
            }
            exec ${wrangler-bin}/bin/wrangler ${wranglerArgs} "$@"
          '';

        # Create a development environment with a script to run wrangler
        hiit-preview = makeWranglerScript {
          name = "hiit-preview";
          wranglerArgs = "dev --env prebuilt --live-reload false";
        };

        # Create a deployment script for Cloudflare
        hiit-deploy = makeWranglerScript {
          name = "hiit-deploy";
          wranglerArgs = "deploy --env prebuilt";
          verbose = true;
        };

        e2e-test = pkgs.writeShellScriptBin "e2e-test" ''
          # Start the web service
          ${hiit-preview}/bin/hiit-preview &
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

        # Clippy check for client code
        hiit-client-clippy = craneLib.cargoClippy {
          inherit src;
          cargoArtifacts = hiit-client-deps;
          cargoClippyExtraArgs = "--target wasm32-unknown-unknown --features hydrate --no-default-features -- --deny warnings";
        };

        # Clippy check for server code
        hiit-server-clippy = craneLib.cargoClippy {
          inherit src;
          cargoArtifacts = hiit-server-deps;
          cargoClippyExtraArgs = "--target wasm32-unknown-unknown --features ssr --no-default-features -- --deny warnings";
        };
      in {
        checks = {
          inherit hiit-client hiit-server;
          inherit hiit-client-clippy hiit-server-clippy;
          hiit-client-fmt = craneLib.cargoFmt {
            inherit src;
          };
          hiit-server-fmt = craneLib.cargoFmt {
            inherit src;
          };
          hiit-toml-fmt = craneLib.taploFmt {
            src = pkgs.lib.sources.sourceFilesBySuffices src [".toml"];
          };
          hiit-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };
          hiit-deny = craneLib.cargoDeny {
            inherit src;
          };
        };

        packages = {
          inherit hiit hiit-client hiit-server hiit-dev;
          e2e = e2e.packages.${system}.default;
          wrangler = wrangler-bin;
          default = hiit;
        };

        apps = rec {
          # Development app for local testing
          preview = {
            type = "app";
            program = "${hiit-preview}/bin/hiit-preview";
            meta.description = "Run HIIT application in local development mode with wrangler";
          };

          default = preview;

          # Deployment app for Cloudflare
          deploy = {
            type = "app";
            program = "${hiit-deploy}/bin/hiit-deploy";
            meta.description = "Deploy HIIT application to Cloudflare Workers";
          };

          # End-to-end test runner
          e2e = {
            type = "app";
            program = "${e2e-test}/bin/e2e-test";
            meta.description = "Run end-to-end tests - requires a local firefox";
          };
        };

        formatter = alejandra.packages.${system}.default;
      }
    );
}
