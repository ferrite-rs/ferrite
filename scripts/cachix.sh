#!/usr/bin/env bash

set -euxo pipefail

nix-store -qR --include-outputs $(nix-instantiate -A project) | cachix push maybevoid
