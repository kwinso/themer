echo "This script will install Themer into /usr/bin and create a directory inside of ~/.config"
read -p "Press [ENTER] to continue installation: "

sudo curl https://gitlab.com/themer-cli/themer/uploads/cc3f4eb318802ae1ed694cde1e78ac24/themer --output /usr/bin/themer && \
sudo chmod +x /usr/bin/themer && \

mkdir -p ~/.config/themer && \
wget https://gitlab.com/themer-cli/themer/-/raw/main/assets/config.yml -O ~/.config/themer/config.yml && \

echo "Themer successfully installed." && \
echo "Check out default configuration file inside of ~/.config/themer/themer.yml"
