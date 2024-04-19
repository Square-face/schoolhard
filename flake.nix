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
            webkitgtk
            gtk3
            cairo
            gdk-pixbuf
            glib
            dbus
            openssl_3
        ];

        packages = with pkgs; [
            cargo
            rustc
            pkg-config
            dbus
            openssl_3
            glib
            gtk3
            libsoup
            webkitgtk_4_1
            appimagekit
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
