#!/usr/bin/env bash
adduce -c config/pages/index -n index.html -o docs
adduce -c config/pages/donate -n donate.html -o docs
adduce -c config/pages/404 -n 404.html -o docs

cp -r config/global/style docs/
