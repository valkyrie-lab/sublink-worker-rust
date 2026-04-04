let
  pkgs = import <nixpkgs> { };
in
pkgs.mkShell {
  buildInputs = [
    pkgs.rustc
    pkgs.cargo
    pkgs.pkg-config
    pkgs.git
    pkgs.wget
    pkgs.xz
    pkgs.ncurses
    pkgs.python3
    pkgs.python3Packages.distutils
    pkgs.gnumake
  ];

  shellHook = ''
    PROJECT_DIR=$(pwd)
    SDK_DIR="$PROJECT_DIR/openwrt-sdk"
    SDK_URL="https://mirrors.tuna.tsinghua.edu.cn/openwrt/releases/23.05.3/targets/ramips/mt7621/openwrt-sdk-23.05.3-ramips-mt7621_gcc-12.3.0_musl.Linux-x86_64.tar.xz"
    SDK_ARCHIVE="$PROJECT_DIR/openwrt-sdk.tar.xz"

    cd "$PROJECT_DIR"

    # === OpenWRT SDK download ===
    if [ ! -d "$SDK_DIR" ]; then
      echo "Downloading OpenWRT SDK from TUNA mirror..."
      wget -O "$SDK_ARCHIVE" "$SDK_URL" || {
        echo "Download failed."
        exit 1
      }
      echo "Extracting SDK..."
      tar -xf "$SDK_ARCHIVE"
      if [ -d "openwrt-sdk-23.05.3-ramips-mt7621" ]; then
        mv openwrt-sdk-23.05.3-ramips-mt7621 "$SDK_DIR"
      elif [ -d "openwrt-sdk-"* ]; then
        mv openwrt-sdk-* "$SDK_DIR"
      fi
      rm -f "$SDK_ARCHIVE"
    fi

    # Copy sublink package files to SDK
    PKG_DIR="$SDK_DIR/package/network/proxy/sublink"
    mkdir -p "$PKG_DIR"
    cp "$PROJECT_DIR/openwrt/Makefile" "$PKG_DIR/"
    cp -r "$PROJECT_DIR/openwrt/files" "$PKG_DIR/"

    # === Build ipk ===
    build-ipk() {
      local OUTPUT_DIR="$SDK_DIR/bin/packages/mipsel_24kc/base"
      local WORK_DIR="/tmp/sublink-build-$$"

      rm -rf "$WORK_DIR"
      mkdir -p "$WORK_DIR/CONTROL"

      # Create directory structure at $WORK_DIR level
      mkdir -p "$WORK_DIR/bin"
      mkdir -p "$WORK_DIR/etc/init.d"
      mkdir -p "$WORK_DIR/etc/sublink"

      cp "$PKG_DIR/files/sublink" "$WORK_DIR/bin/sublink"
      chmod 755 "$WORK_DIR/bin/sublink"

      cp "$PKG_DIR/files/sublink.init" "$WORK_DIR/etc/init.d/sublink"
      chmod 755 "$WORK_DIR/etc/init.d/sublink"

      cp "$PKG_DIR/files/config.toml.example" "$WORK_DIR/etc/sublink/config.toml"

      # Create control file
      cat > "$WORK_DIR/CONTROL/control" << EOF
Package: sublink
Version: 1.0.0-1
Description: Sublink Worker - A lightweight proxy worker service for sublinks
Section: net
Priority: optional
Maintainer: unknown
Architecture: mipsel_24kc
License: MIT
EOF

      # Create postinst script - enables service and cleans up temp files
      cat > "$WORK_DIR/CONTROL/postinst" << 'POSTINST'
#!/bin/sh
# Clean up temp files left by opkg
rm -f /data.tar.gz /control.tar.gz /debian-binary 2>/dev/null
rm -f /data /control.tar.gz /debian-binary 2>/dev/null

# Enable service
[ -n "$IPKG_INSTROOT" ] && exit 0
/etc/init.d/sublink enable 2>/dev/null || true
exit 0
POSTINST
      chmod 755 "$WORK_DIR/CONTROL/postinst"

      # Create prerm script
      cat > "$WORK_DIR/CONTROL/prerm" << 'PRERM'
#!/bin/sh
[ -n "$IPKG_INSTROOT" ] && exit 0
/etc/init.d/sublink disable 2>/dev/null || true
exit 0
PRERM
      chmod 755 "$WORK_DIR/CONTROL/prerm"

      # Create data.tar.gz - must contain ./bin, ./etc paths
      cd "$WORK_DIR"
      tar -czvf data.tar.gz bin etc

      # Create control.tar.gz with control, postinst, prerm
      tar -czvf control.tar.gz CONTROL

      # Create debian-binary
      echo "2.0" > debian-binary

      # Create ipk
      rm -f "$OUTPUT_DIR/sublink_1.0.0-1_mipsel_24kc.ipk"
      "$SDK_DIR/scripts/ipkg-build" . "$OUTPUT_DIR"
      mv "$OUTPUT_DIR/sublink_1.0.0-1_mipsel_24kc.ipk" "$PROJECT_DIR/"

      rm -rf "$WORK_DIR"
      echo ""
      echo "=== IPK created ==="
      ls -la "$PROJECT_DIR/sublink_1.0.0-1_mipsel_24kc.ipk"
    }

    echo ""
    echo "=== Build ipk ==="
    echo "Run: build-ipk"
    echo "Output: <project>/sublink_1.0.0-1_mipsel_24kc.ipk"
  '';
}
