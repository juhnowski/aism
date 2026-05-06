{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  nativeBuildInputs = with pkgs; [
    rustc
    cargo
    trunk
    wasm-bindgen-cli
    pkg-config
    openssl
    # Добавьте эти пакеты:
    llvmPackages.bintools 
    lld
  ];

  shellHook = ''
    rustup target add wasm32-unknown-unknown 2>/dev/null || true
    echo "--- Среда разработки готова (с линкером LLD) ---"
  '';
}
