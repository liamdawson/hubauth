#!/usr/bin/env bash

set -eu

apt-get update -y
apt-get install -y \
  build-essential \
  pbuilder \
  git-buildpackage

pbuilder --create --distribution bionic --architecture amd64 --basetgz /var/cache/pbuilder/bionic-amd64-base.tgz
pbuilder --create --distribution cosmic --architecture amd64 --basetgz /var/cache/pbuilder/cosmic-amd64-base.tgz
