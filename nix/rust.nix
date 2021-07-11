# nix/rust.nix
{ sources ? import ./sources.nix }:

let
  pkgs =
    import sources.nixpkgs { overlays = [ (import sources.rust-overlay) ]; };
  channel = "nightly";
  date = "2021-03-08";
  targets = [ ];
  chan = pkgs.rustChannelOfTargets channel date targets;
in chan

