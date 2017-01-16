#!/bin/bash

FOLDER="."
FILES='.+\.\(md\|rs\|\sh\|toml\|txt\|yml\)'

# Exit script on first error
set -o errexit -o nounset

# Explain what we do
echo -n ">>> Seaching for files with tab characters..."

# Search for files with tab characters
FOUND=0
for FILE in $(find $FOLDER -regex $FILES); do
    if grep -q $'\t' $FILE ; then
        echo -e "\nFound: $FILE"
        FOUND=1
    fi
done
test $FOUND == 0
echo -e "\t\tDone."
