echo "This script will install Themer into /usr/bin and create a directory inside of ~/.config"
read -p "Press [ENTER] to continue installation: "

sudo curl https://gitlab.com/themer-cli/themer/uploads/3700376c865c55ecf51d5e3954587763/themer -s --output /usr/bin/themer && \
sudo chmod +x /usr/bin/themer && \

mkdir -p ~/.config/themer

CONFIG=~/.config/themer/config.yml
if [ ! -f "$CONFIG" ]; then
  echo "Making default config in $CONFIG"
  wget -q https://gitlab.com/themer-cli/themer/-/raw/main/assets/config.yml -O $CONFIG
fi


echo "Themer successfully installed." && \
echo "Check out default configuration file inside of $CONFIG"
