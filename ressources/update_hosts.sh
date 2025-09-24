#!/bin/bash

set -e

HOSTS_FILE="/etc/hosts"
CONFIG_FILE="./etc/config.txt"
DOMAIN="myserver.test"

# Extract the first non-loopback IPv4
IP=$(ip -4 addr show | awk '/inet / && $2 !~ /^127/ {print $2}' | awk -F/ 'NR==1{print $1}')

if [ -z "$IP" ]; then
  echo "FAILED"
  exit 1
fi

LINE="$IP    $DOMAIN"

# --- Update /etc/hosts ---
awk -v newline="$LINE" -v domain="$DOMAIN" '
    NR==1 {print; next}
    NR==2 {print; print newline; next}
    # skip any line containing the domain (removes duplicates)
    $0 ~ domain {next}
    {print}
' "$HOSTS_FILE" > /tmp/hosts.new

if cmp -s "$HOSTS_FILE" /tmp/hosts.new; then
    rm /tmp/hosts.new
    echo "ALREADY_OK"
else
    sudo mv /tmp/hosts.new "$HOSTS_FILE"
    echo "UPDATED"
fi

# --- Update config.txt ---
awk -v ip="$IP" -v domain="$DOMAIN" '
    # replace any line containing the domain with "IP:7879 domain"
    $0 ~ domain {
        print ip ":7879 " domain
        next
    }
    {print}
' "$CONFIG_FILE" > /tmp/config.new

if cmp -s "$CONFIG_FILE" /tmp/config.new; then
    rm /tmp/config.new
    echo "CONFIG_ALREADY_OK"
else
    mv /tmp/config.new "$CONFIG_FILE"
    echo "CONFIG_UPDATED"
fi
