# build me with
# nix-build -E "with import <nixpkgs> {}; callPackage ./default.nix {}"
{ stdenv
, rustPlatform
, fetchFromGitHub
, llvmPackages
, pkgconfig
, makeWrapper
# WIP, try to make the package buildable both locally and from github
#, src ? (fetchFromGitHub {
#    owner = "AberDerBart";
#    repo = "shop-rs";
#    rev = "master";
#    hash = "sha256:0000000000000000000000000000000000000000000000000000";
#  })
}:

rustPlatform.buildRustPackage rec {
  pname = "shop-rs";
  version = "0.1.0";

  #inherit src;
  src = ./.;
  #src = fetchFromGitHub {
  #  owner = "AberDerBart";
  #  repo = "shop-rs";
  #  rev = "6d536b2dca978b4003d3839e28608954edf091c4";
  #  hash = "sha256:0000000000000000000000000000000000000000000000000000";
  #};

  cargoSha256 = "sha256:0000000000000000000000000000000000000000000000000000";

  meta = with stdenv.lib; {
    description = "A command line client for shoppinglist.";
    homepage = "https://github.com/AberDerBart/shop-rs";
    license = with licenses; [ mit ];
    maintainers = with maintainers; [ puzzlewolf aberDerBart ];
    platforms = platforms.all;
  };
}
