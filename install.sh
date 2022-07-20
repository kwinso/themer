echo "This script will install Themer into /usr/bin and create a directory inside of ~/.config"
read -p "Press [ENTER] to continue installation: "

sudo curl https://github.com/uwumouse/themer/releases/latest/download/themer -s --output /usr/bin/themer && \
sudo chmod +x /usr/bin/themer && \

mkdir -p ~/.config/themer

CONFIG=~/.config/themer/config.yml
if [ ! -f "$CONFIG" ]; then
  echo "Making default config in $CONFIG"
  wget -q https://github.com/uwumouse/themer/releases/latest/download/config.yml -O $CONFIG
fi


echo "Themer successfully installed." && \
echo "Check out default configuration file inside of $CONFIG"
