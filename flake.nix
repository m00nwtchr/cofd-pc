{
  description = "A devShell example";
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustBin = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in {
        devShells.default = with pkgs;
          mkShell rec {
            nativeBuildInputs = [pkg-config];
            buildInputs = [
              (rustBin.override {
                extensions = ["rust-docs" "rust-src" "clippy"];
              })
              (rust-bin.selectLatestNightlyWith (toolchain:
                  toolchain.rustfmt))

              libxkbcommon
              libGL
              wayland

              zsh
            ];

            CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER = "${pkgs.llvmPackages.clangUseLLVM}/bin/clang";
            RUSTFLAGS = "-Clink-arg=-fuse-ld=${pkgs.mold}/bin/mold";
            LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";

            shellHook = ''
              exec zsh
            '';
          };
        packages.default = with pkgs; let
          rustPlatform = makeRustPlatform {
            cargo = rustBin;
            rustc = rustBin;
          };
        in
          rustPlatform.buildRustPackage rec {
            pname = "cofd-pc";
            version = "1.0.0";
            src = ./.;
            cargoBuildFlags = "-p cofd-pc";

            cargoLock = {
              lockFile = ./Cargo.lock;
              allowBuiltinFetchGit = true;
            };

            LD_LIBRARY_PATH = "${lib.makeLibraryPath buildInputs}";

            nativeBuildInputs = [pkg-config];
            buildInputs = [
              libGL
              libxkbcommon
              wayland
            ];
          };
      }
    );
}
