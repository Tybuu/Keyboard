{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    rust-overlay,
    nixpkgs,
  }: let
    overlays = [(import rust-overlay)];
    pkgs = import nixpkgs {
      system = "x86_64-linux";
      inherit overlays;
    };
  in {
    packages.x86_64-linux.elf2uf2-rs = pkgs.callPackage ./elf.nix {};
    devShell.x86_64-linux = pkgs.mkShell {
      buildInputs = [
        (pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            targets = ["thumbv6m-none-eabi"];
            extensions = ["rust-src" "rust-analyzer"];
          }))
        # pkgs.rust-analyzer
        pkgs.flip-link
        pkgs.probe-rs
        self.packages.x86_64-linux.elf2uf2-rs
        pkgs.rustfmt
      ];
    };
  };
}
