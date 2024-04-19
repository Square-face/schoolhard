{
    inputs = {
        nixpkgs.url = "github:nixos/nixpkgs";
        flake-utils.url = "github:numtide/flake-utils";
    };

    outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
    let
        pkgs = nixpkgs.legacyPackages.${system};

        libraries = with pkgs;[
            webkitgtk_4_1
            gtk3
            cairo
            gdk-pixbuf
            glib.out
            dbus.lib
            openssl_3.out
        ];

        packages = with pkgs; [
            cargo
            rustc
            rustup
            pkg-config
            dbus
            openssl_3
            glib
            gtk3
            libsoup_3
            webkitgtk_4_1
            appimagekit
            llvmPackages.bintools
        ];
    in
    {
    devShell = pkgs.mkShell {
        buildInputs = packages;

        shellHook = let
            joinLibs = libs: builtins.concatStringsSep ":" (builtins.map (x: "${x}/lib") libs);
            libs = joinLibs libraries;
        in
            ''
            export LD_LIBRARY_PATH=${libs}:$LD_LIBRARY_PATH
            '';
    };});
}
