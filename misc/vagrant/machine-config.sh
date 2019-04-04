#!/usr/bin/env bash

set -eu

apt-get update -y
apt-get install -y \
  build-essential \
  pbuilder \
  debootstrap \
  devscripts \
  libssl-dev \
  libssl1.1 \
  pkg-config \
  debhelper \
  autoconf

pbuilder create --debootstrapopts --variant=buildd

pbuilder --create --distribution bionic --architecture amd64 --basetgz /var/cache/pbuilder/bionic-amd64-base.tgz
pbuilder --create --distribution cosmic --architecture amd64 --basetgz /var/cache/pbuilder/cosmic-amd64-base.tgz
