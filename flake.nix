{
  description = "CoolerControl custom-device (IPMI) plugin";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in
    {
      packages.${system} = {
        default = pkgs.rustPlatform.buildRustPackage {
          name = "custom-device";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.protobuf ];
          installPhase = ''
            runHook preInstall
            mkdir -p $out/bin $out/plugin-files
            cp target/x86_64-unknown-linux-gnu/release/custom-device $out/bin/
            cp -r plugin-files/* $out/plugin-files/
            runHook postInstall
          '';
        };
      };
    };
}
