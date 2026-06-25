#!/usr/bin/env sh
set -eu

REPO=${REDMINE_MCP_REPO:-weirdo-adam/redmine-mcp-server}
BINARY_NAME=redmine-mcp-server
INSTALL_DIR=${REDMINE_MCP_INSTALL_DIR:-"$HOME/.local/bin"}

download_stdout() {
  url=$1

  if command -v curl >/dev/null 2>&1; then
    curl -fsSL -H "User-Agent: redmine-mcp-server-installer" "$url"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO- --header="User-Agent: redmine-mcp-server-installer" "$url"
  else
    echo "curl or wget is required to download Redmine MCP server." >&2
    exit 1
  fi
}

download_file() {
  url=$1
  output=$2

  if command -v curl >/dev/null 2>&1; then
    curl -fsSL -H "User-Agent: redmine-mcp-server-installer" "$url" -o "$output"
  elif command -v wget >/dev/null 2>&1; then
    wget -q --header="User-Agent: redmine-mcp-server-installer" "$url" -O "$output"
  else
    echo "curl or wget is required to download Redmine MCP server." >&2
    exit 1
  fi
}

resolve_tag() {
  if [ -n "${REDMINE_MCP_VERSION:-}" ]; then
    case "$REDMINE_MCP_VERSION" in
      v*) printf '%s\n' "$REDMINE_MCP_VERSION" ;;
      *) printf 'v%s\n' "$REDMINE_MCP_VERSION" ;;
    esac
    return
  fi

  download_stdout "https://api.github.com/repos/$REPO/releases/latest" \
    | sed -n 's/^[[:space:]]*"tag_name":[[:space:]]*"\([^"]*\)".*/\1/p' \
    | head -n 1
}

detect_os() {
  os=$(uname -s)
  case "$os" in
    Darwin) printf 'macos\n' ;;
    Linux) printf 'linux\n' ;;
    MINGW*|MSYS*|CYGWIN*) printf 'windows\n' ;;
    *)
      echo "Unsupported operating system: $os" >&2
      exit 1
      ;;
  esac
}

detect_arch() {
  arch=$(uname -m)
  case "$arch" in
    x86_64|amd64) printf 'x86_64\n' ;;
    arm64|aarch64) printf 'aarch64\n' ;;
    *)
      echo "Unsupported CPU architecture: $arch" >&2
      exit 1
      ;;
  esac
}

sha256_file() {
  file=$1

  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$file" | awk '{print $1}'
  elif command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$file" | awk '{print $1}'
  else
    echo "shasum or sha256sum is required to verify Redmine MCP server." >&2
    exit 1
  fi
}

tag=$(resolve_tag)
if [ -z "$tag" ]; then
  echo "Unable to resolve the latest Redmine MCP server release." >&2
  exit 1
fi

version=${tag#v}
os=$(detect_os)
arch=$(detect_arch)
exe_suffix=
if [ "$os" = "windows" ]; then
  exe_suffix=.exe
fi

package="$BINARY_NAME-$version-$os-$arch.tar.gz"
base_url="https://github.com/$REPO/releases/download/$tag"
tmp_dir=$(mktemp -d "${TMPDIR:-/tmp}/redmine-mcp-server-install.XXXXXX")
trap 'rm -rf "$tmp_dir"' EXIT HUP INT TERM

archive="$tmp_dir/$package"
checksum="$tmp_dir/$package.sha256"

echo "Installing Redmine MCP server $version for $os-$arch..."
download_file "$base_url/$package" "$archive"
download_file "$base_url/$package.sha256" "$checksum"

expected=$(sed 's/[[:space:]].*//' "$checksum" | head -n 1)
actual=$(sha256_file "$archive")
if [ "$expected" != "$actual" ]; then
  echo "Checksum verification failed for $package" >&2
  echo "Expected: $expected" >&2
  echo "Actual:   $actual" >&2
  exit 1
fi

tar -xzf "$archive" -C "$tmp_dir"

mkdir -p "$INSTALL_DIR"
cp "$tmp_dir/$BINARY_NAME$exe_suffix" "$INSTALL_DIR/$BINARY_NAME$exe_suffix"
chmod +x "$INSTALL_DIR/$BINARY_NAME$exe_suffix"

echo
echo "Installed Redmine MCP server to:"
echo "  $INSTALL_DIR/$BINARY_NAME$exe_suffix"
echo
echo "Required environment:"
echo "  REDMINE_BASE_URL=https://redmine.example.com"
echo "  REDMINE_API_KEY=your-api-key"
echo
case ":${PATH:-}:" in
  *":$INSTALL_DIR:"*) ;;
  *)
    echo "Add this directory to PATH if your MCP client cannot find the command:"
    echo "  $INSTALL_DIR"
    echo
    ;;
esac
echo "MCP command:"
echo "  $BINARY_NAME$exe_suffix"
