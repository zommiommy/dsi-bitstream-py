{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        python = pkgs.python3;
        target = pkgs.stdenv.hostPlatform.rust.rustcTargetSpec;

        cargoVendor = pkgs.rustPlatform.fetchCargoVendor {
          src = ./.;
          hash = "sha256-j3wIG/bKzO+yNakBXmVBaNpeU6F8mjQrUF5XL2J2DpY=";
        };

        # Shared build inputs for cargo vendoring + maturin.
        commonNativeBuildInputs = [
          pkgs.rustPlatform.cargoSetupHook
          pkgs.maturin
          pkgs.cargo
          pkgs.rustc
        ];

        # Shared build phase logic. Takes the maturin compatibility + extra
        # flags and produces the wheel into $PWD/dist.
        #
        # We bypass maturinBuildHook entirely because it hardcodes
        # `--manylinux off` and there is no way to override that.
        mkBuildPhase =
          {
            compatibility,
            extraFlags ? "",
          }:
          ''
            runHook preBuild

            local dist="$PWD/dist"
            local interpreter_path="$(command -v python3)"
            local interpreter_name
            interpreter_name="$($interpreter_path -c \
              'import os, sysconfig; print(os.path.basename(sysconfig.get_config_var("INCLUDEPY")))')"

            # Unset _PYTHON_HOST_PLATFORM (set by nixpkgs python hook) so
            # maturin controls the platform tag.
            unset _PYTHON_HOST_PLATFORM

            export HOME="$TMPDIR"
            maturin build \
              --jobs=$NIX_BUILD_CORES \
              --offline \
              --target ${target} \
              --compatibility ${compatibility} \
              --strip \
              --release \
              --out "$dist" \
              --interpreter "$interpreter_name" \
              ${extraFlags}

            runHook postBuild
          '';
      in
      {
        packages = {
          # Native wheel tagged `linux_x86_64`. Installs on the host system.
          default = python.pkgs.buildPythonPackage {
            pname = "dsi_bitstream";
            version = "0.9.1";
            pyproject = true;
            src = ./.;
            cargoDeps = cargoVendor;
            nativeBuildInputs = commonNativeBuildInputs;
            buildPhase = mkBuildPhase { compatibility = "linux"; };
            pythonImportsCheck = [ "dsi_bitstream" ];
          };

          # manylinux2014 wheel built with zig as the linker. Zig ships its
          # own glibc 2.17 headers/stubs, so the resulting .so only uses
          # symbols available in glibc 2.17 — no Docker container needed.
          # Uploadable to PyPI.
          manylinux = python.pkgs.buildPythonPackage {
            pname = "dsi_bitstream";
            version = "0.9.1";
            pyproject = true;
            src = ./.;
            cargoDeps = cargoVendor;
            nativeBuildInputs = commonNativeBuildInputs ++ [
              pkgs.zig
              pkgs.cargo-zigbuild
              python.pkgs.auditwheel
            ];
            buildPhase = mkBuildPhase {
              compatibility = "manylinux2014";
              extraFlags = "--zig";
            };

            # Verify the wheel is genuinely manylinux2014-compliant.
            postBuild = ''
              auditwheel show "$PWD"/dist/*.whl
            '';

            pythonImportsCheck = [ "dsi_bitstream" ];
          };
        };

        devShells.default = pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.rustc
            pkgs.maturin
            pkgs.patchelf
            pkgs.zig
            pkgs.cargo-zigbuild
            python
          ];
        };
      }
    );
}
