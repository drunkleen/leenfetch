#!/bin/sh
set -eu

REPO="drunkleen/leenfetch"
BIN_DIR="/usr/local/bin"
CONFIG_DIR="${HOME}/.config/leenfetch"

usage() {
    cat <<'EOF'
Usage: curl -sSfL https://github.com/drunkleen/leenfetch/releases/latest/download/install.sh | sudo sh [-- [--update|--force]]

Commands:
  (none)    Install leenfetch (aborts if already installed)
  --update  Update leenfetch to latest version (if newer)
  --force   Reinstall even if already at latest version

Environment:
  LEENFETCH_VERSION  Install a specific version (e.g. v1.2.2)
EOF
    exit 0
}

MODE="install"
for arg in "$@"; do
    case "$arg" in
        --help|-h) usage ;;
        --update) MODE="update" ;;
        --force) MODE="force" ;;
    esac
done

if command -v leenfetch >/dev/null 2>&1 && [ "$MODE" = "install" ]; then
    echo "leenfetch is already installed. Use --update to upgrade or --force to reinstall."
    exit 0
fi

echo "==> Detecting system..."

ARCH=$(uname -m)
case "$ARCH" in
    x86_64|amd64) ARCH="x86_64" ;;
    aarch64|arm64) ARCH="aarch64" ;;
    *)
        echo "Unsupported architecture: $ARCH"
        exit 1
        ;;
esac

DISTRO=""
if [ -f /etc/os-release ]; then
    . /etc/os-release
    case "$ID" in
        debian|ubuntu|linuxmint|pop|elementary|zorin) DISTRO="debian" ;;
        fedora|rhel|centos|rocky|almalinux) DISTRO="rhel" ;;
        arch|manjaro|endeavouros|garuda|artix) DISTRO="arch" ;;
        alpine) DISTRO="alpine" ;;
        *) DISTRO="tar-gz" ;;
    esac
elif [ -f /etc/arch-release ]; then
    DISTRO="arch"
elif [ -f /etc/redhat-release ]; then
    DISTRO="rhel"
elif [ -f /etc/debian_version ]; then
    DISTRO="debian"
else
    DISTRO="tar-gz"
fi

# AUR helper: install directly if available
if [ "$DISTRO" = "arch" ]; then
    if command -v yay >/dev/null 2>&1; then
        echo "==> Installing via yay from AUR..."
        yay -S --noconfirm leenfetch
        exit $?
    fi
    if command -v paru >/dev/null 2>&1; then
        echo "==> Installing via paru from AUR..."
        paru -S --noconfirm leenfetch
        exit $?
    fi
fi

# Determine package name and type
case "$DISTRO" in
    debian) PKG="debian-${ARCH}.deb"   TYPE="deb" ;;
    rhel)   PKG="rhel-${ARCH}.rpm"     TYPE="rpm" ;;
    arch)   PKG="arch-${ARCH}.pkg.tar.zst" TYPE="arch" ;;
    alpine) PKG="linux-${ARCH}.tar.gz" TYPE="tar-gz" ;;
    *)      PKG="linux-${ARCH}.tar.gz" TYPE="tar-gz" ;;
esac

# glibc version check — fall back to static musl binary on old systems
GLIBC_VER=$(ldd --version 2>/dev/null | head -1 | grep -oiE 'glibc [0-9]+\.[0-9]+' | cut -d' ' -f2 || echo "")
if [ -n "$GLIBC_VER" ]; then
    GLIBC_MAJOR=$(echo "$GLIBC_VER" | cut -d. -f1)
    GLIBC_MINOR=$(echo "$GLIBC_VER" | cut -d. -f2)
    if [ "$GLIBC_MAJOR" -lt 2 ] || { [ "$GLIBC_MAJOR" -eq 2 ] && [ "$GLIBC_MINOR" -lt 39 ]; }; then
        echo "==> Detected glibc ${GLIBC_VER} < 2.39 — using static musl binary"
        PKG="linux-${ARCH}-musl.tar.gz"
        TYPE="tar-gz"
    fi
fi

# Fetch latest version from GitHub API
if [ -z "${LEENFETCH_VERSION:-}" ]; then
    echo "==> Fetching latest version..."
    LEENFETCH_VERSION=$(curl -sSfL --retry 3 --retry-delay 2 \
        "https://api.github.com/repos/${REPO}/releases/latest" | \
        grep '"tag_name"' | cut -d'"' -f4)
    if [ -z "$LEENFETCH_VERSION" ]; then
        echo "Failed to fetch latest version. Check your internet connection."
        exit 1
    fi
fi

# Version comparison for --update mode
if [ "$MODE" = "update" ] && command -v leenfetch >/dev/null 2>&1; then
    LOCAL_VER=$(leenfetch --version 2>/dev/null | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1 || echo "0.0.0")
    REMOTE_VER="${LEENFETCH_VERSION#v}"
    if [ "$LOCAL_VER" = "$REMOTE_VER" ]; then
        echo "Already at latest version ($LOCAL_VER)."
        exit 0
    fi
    echo "==> Updating from $LOCAL_VER to $REMOTE_VER"
fi

echo "==> Resolving download URL..."
DOWNLOAD_URL=$(curl -s --retry 3 --retry-delay 2 \
    "https://api.github.com/repos/${REPO}/releases/tags/${LEENFETCH_VERSION}" | \
    grep -o "\"browser_download_url\": \"[^\"]*${PKG}[^\"]*\"" | cut -d'"' -f4)
if [ -z "$DOWNLOAD_URL" ]; then
    DOWNLOAD_URL=$(curl -s --retry 3 --retry-delay 2 \
        "https://api.github.com/repos/${REPO}/releases/latest" | \
        grep -o "\"browser_download_url\": \"[^\"]*${PKG}[^\"]*\"" | cut -d'"' -f4)
fi
if [ -z "$DOWNLOAD_URL" ]; then
    echo "Failed to resolve download URL for ${PKG}."
    echo "Try downloading manually from: https://github.com/${REPO}/releases/tag/${LEENFETCH_VERSION}"
    exit 1
fi

TMPDIR=$(mktemp -d)
trap "rm -rf \"${TMPDIR}\"" EXIT

DOWNLOADED="${TMPDIR}/leenfetch.${TYPE}"

# Download with retry and verify
echo "==> Downloading..."
curl -sSfL --retry 3 --retry-delay 2 --retry-connrefused \
    -o "${DOWNLOADED}" "${DOWNLOAD_URL}"

SIZE=$(stat -c%s "${DOWNLOADED}" 2>/dev/null || echo "0")
if [ "$SIZE" -lt 1024 ]; then
    echo "Downloaded file is too small (${SIZE} bytes). Corrupted download?"
    exit 1
fi

# Root check: use sudo only when not already root
if [ "$(id -u)" -eq 0 ]; then
    SUDO=""
else
    SUDO="sudo"
fi

# Configuration backup
CONFIG_BAK="${CONFIG_DIR}/config.jsonc.bak"
if [ -f "${CONFIG_DIR}/config.jsonc" ]; then
    mkdir -p "${CONFIG_DIR}"
    cp "${CONFIG_DIR}/config.jsonc" "${CONFIG_BAK}"
fi

echo "==> Installing..."

case "$TYPE" in
    deb)
        if [ -n "$SUDO" ]; then
            $SUDO dpkg -i "${DOWNLOADED}"
            $SUDO apt-get install -f -y >/dev/null 2>&1 || true
        else
            mkdir -p "${BIN_DIR}"
            dpkg --fsys-tarfile "${DOWNLOADED}" | tar -xzf - -C "${TMPDIR}" ./usr/bin/leenfetch 2>/dev/null || true
            cp "${TMPDIR}/usr/bin/leenfetch" "${BIN_DIR}/leenfetch" 2>/dev/null || cp "${TMPDIR}/bin/leenfetch" "${BIN_DIR}/leenfetch" 2>/dev/null || true
            if [ ! -f "${BIN_DIR}/leenfetch" ]; then
                ar p "${DOWNLOADED}" data.tar.zst 2>/dev/null | zstd -d 2>/dev/null | tar -xzf - -C "${TMPDIR}" ./usr/bin/leenfetch 2>/dev/null || true
                cp "${TMPDIR}/usr/bin/leenfetch" "${BIN_DIR}/leenfetch" 2>/dev/null || true
            fi
            [ -f "${BIN_DIR}/leenfetch" ] || { echo "Extraction failed."; exit 1; }
        fi
        ;;
    rpm)
        if [ -n "$SUDO" ]; then
            $SUDO rpm -i "${DOWNLOADED}" 2>/dev/null || $SUDO dnf install -y "${DOWNLOADED}" 2>/dev/null || $SUDO yum install -y "${DOWNLOADED}" 2>/dev/null || true
        else
            mkdir -p "${BIN_DIR}"
            rpm2cpio "${DOWNLOADED}" 2>/dev/null | cpio -idm -D "${TMPDIR}" ./usr/bin/leenfetch 2>/dev/null || true
            cp "${TMPDIR}/usr/bin/leenfetch" "${BIN_DIR}/leenfetch" 2>/dev/null || true
            [ -f "${BIN_DIR}/leenfetch" ] || { echo "Extraction failed."; exit 1; }
        fi
        ;;
    arch)
        if [ -n "$SUDO" ]; then
            $SUDO pacman -U --noconfirm "${DOWNLOADED}" 2>/dev/null || true
        else
            mkdir -p "${BIN_DIR}"
            tar -xaf "${DOWNLOADED}" -C "${TMPDIR}" ./usr/bin/leenfetch 2>/dev/null || true
            cp "${TMPDIR}/usr/bin/leenfetch" "${BIN_DIR}/leenfetch" 2>/dev/null || true
            [ -f "${BIN_DIR}/leenfetch" ] || { echo "Extraction failed."; exit 1; }
        fi
        ;;
    tar-gz)
        mkdir -p "${BIN_DIR}"
        tar -xzf "${DOWNLOADED}" -C "${BIN_DIR}" leenfetch 2>/dev/null || true
        if [ ! -f "${BIN_DIR}/leenfetch" ]; then
            tar -xzf "${DOWNLOADED}" -C "${TMPDIR}"
            cp "${TMPDIR}/leenfetch" "${BIN_DIR}/leenfetch" 2>/dev/null || true
        fi
        [ -f "${BIN_DIR}/leenfetch" ] || { echo "Installation failed."; exit 1; }
        ;;
esac

# Restore config from backup (overwrites any newly generated default)
if [ -f "${CONFIG_BAK}" ]; then
    mkdir -p "${CONFIG_DIR}"
    mv "${CONFIG_BAK}" "${CONFIG_DIR}/config.jsonc"
fi

# Final success message and PATH warning
if command -v leenfetch >/dev/null 2>&1 || [ -f "${BIN_DIR}/leenfetch" ]; then
    INSTALLED_VER=$(leenfetch --version 2>/dev/null | head -1 || echo "installed")
    echo "==> Successfully installed leenfetch ${LEENFETCH_VERSION} (${INSTALLED_VER})"
else
    echo "==> leenfetch installed to ${BIN_DIR}/leenfetch"
    case ":${PATH}:" in
        *:"${BIN_DIR}":*) ;;
        *)
            echo "    Add ${BIN_DIR} to your PATH:"
            echo "    export PATH=\"${BIN_DIR}:\$PATH\"   # bash/zsh"
            echo "    fish_add_path ${BIN_DIR}              # fish"
            ;;
    esac
fi

# Suggest --init if no config exists
if [ ! -f "${CONFIG_DIR}/config.jsonc" ] && command -v leenfetch >/dev/null 2>&1; then
    echo "==> Run 'leenfetch --init' to create a default config file"
fi
echo "==> Run 'leenfetch' to get started!"
