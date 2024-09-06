{
    inputs = {
        flake-compat.url = "https://flakehub.com/f/edolstra/flake-compat/1.tar.gz";
        flake-utils.url = "github:numtide/flake-utils";
        fenix = {
            url = "github:nix-community/fenix";
            inputs.nixpkgs.follows = "nixpkgs";
        };
        nixpkgs.url = "nixpkgs/nixos-unstable";
    };

    outputs = { self, nixpkgs, flake-utils, fenix, ...}:
        flake-utils.lib.eachDefaultSystem (system:
            let
                overlays = [fenix.overlays.default];
                pkgs = import nixpkgs {
                    inherit system overlays;
                };
                rust' = fenix.packages.${system}.fromToolchainFile { 
                    file = ./rust-toolchain.toml;
                    sha256 = "sha256-sZ4gSN88DqNWcUSUloG3tX8hZulnsvmtkRIpWMPPzBg=";
                };
                nativeBuildInputs = with pkgs; [
                    clang
                    llvmPackages.bintools
                    pkg-config
                    rust'
                ];
                buildInputs = with pkgs; [
                    rust-analyzer-nightly
                    cargo-watch

                    udev alsa-lib vulkan-loader
                    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr # To use the x11 feature
                    libxkbcommon wayland # To use the wayland feature
                ];
                allInputs = nativeBuildInputs ++ buildInputs;
                env = with pkgs; {
                    LIBCLANG_PATH = lib.makeLibraryPath [
                        llvmPackages_latest.libclang.lib
                    ];
                    RUSTFLAGS = (builtins.map(a: ''-L ${a}/lib'') []);
                    LD_LIBRARY_PATH = lib.makeLibraryPath allInputs;
                    BINGEN_EXTRA_CLANG_ARGS = (builtins.map(a: ''-I"${a}/include"'') [
                        pkgs.glibc.dev
                    ]) ++ [
                        ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
                        ''-I"${pkgs.glib.dev}/include/glib-2.0"''
                        ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
                    ];
                };
                shell = rec {
                    inherit nativeBuildInputs buildInputs;

                    shellHook = ''
                        export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
                        export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
                    '';
                } // env;
            in
                {
                    devShells.default = pkgs.mkShell shell;
                }
        );
}