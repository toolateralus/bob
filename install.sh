if [ "$EUID" -ne 0 ]; then
  echo "Error: This script must be run as root or with sudo privileges."
  exit 1
fi

if [ ! -f target/release/bob ]; then
  echo -e "\e[1;31mError: No target found! run \`cargo build --release\` first!\e[0m"
  exit 1
fi

echo -e "\e[1;33mCopying binary to /usr/local/bin/bob...\e[0m"
sudo cp target/release/bob /usr/local/bin
echo -e "\e[1;32mDone!\e[0m"
