{
  description = "fai dev env";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { nixpkgs, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };

    in {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rustc
          cargo
          rustfmt
          openssl
          pkg-config
        ];

        shellHook = ''
          echo "Rust version: $(rustc --version)"
          echo "Cargo version: $(cargo --version)"
          echo "OpenSSL version: $(openssl --version)"
        '';
    };
  };
}
