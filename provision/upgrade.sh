#!/bin/bash

DEBIAN_FRONTEND=noninteractive \
apt-get -q -y upgrade \
-o Dpkg::Options::="--force-confdef" \
-o Dpkg::Options::="--force-confold"

apt-get autoremove -y