BLUE='\033[0;34m'
GREEN='\033[0;32m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

TAG="v1.3.0"

function download_config() {
  mkdir -p ~/.config/themer

  CONFIG=~/.config/themer/config.yml
  if [ ! -f "$CONFIG" ]; then
    echo
    echo "Themer config is not found in $CONFIG"
    while true; do
      echo -e -n "Would you like to create default one? $PURPLE[Y/N]$NC "
      read yn
      case $yn in
          [Yy]* ) 
            echo -e "${BLUE}Downloading default config to $CONFIG... $NC"
            wget https://github.com/uwumouse/themer/releases/download/$TAG/config.yml -q --show-progress -O $CONFIG && \
            echo -e "Default config is dowloaded to: ${BLUE}$CONFIG$NC"
            break;;
          [Nn]* ) break;;
          * ) echo -e "Please answer \"${BLUE}Y$NC\" or \"${BLUE}N$NC\".";;
      esac
  done
  fi
}

function print_success() {
  echo
  echo -e "${GREEN}Themer successfully installed!"
}

echo "This script will install Themer into /usr/bin and create a directory inside of ~/.config"
echo -e -n "Press $PURPLE[ENTER]$NC to continue installation: "
read

echo -e "${BLUE}Downloading Themer binary... $NC"

sudo wget https://github.com/uwumouse/themer/releases/download/$TAG/themer -q --show-progress -O /usr/bin/themer && \
sudo chmod +x /usr/bin/themer && \
download_config && print_success
