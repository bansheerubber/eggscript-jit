{ pkgs ? import <nixpkgs> {} }:
let unstable =
	import <nixos-unstable> { config = { allowUnfree = true; }; };
in pkgs.mkShell {
	nativeBuildInputs = with pkgs.buildPackages; [ unstable.rustup ];

	shellHook = ''
		rustup default nightly
	'';
}
