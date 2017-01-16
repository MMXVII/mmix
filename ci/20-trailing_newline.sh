#!/bin/bash

FOLDER="."
FILES='.+\.\(md\|rs\|\sh\|toml\|txt\|yml\)'

# Exit script on first error
set -o errexit -o nounset

# Explain what we do
echo -n ">>> Seaching for files without a trailing newline..."

# Search for trailing newline
FOUND=0
for FILE in $(find $FOLDER -regex $FILES); do
    LASTLINE=$(tail -n 1 $FILE; echo x)
    LASTLINE=${LASTLINE%x}
    if [ "${LASTLINE: -1}" != $'\n' ] ; then
        echo -e "\nFound:\t$FILE"
        FOUND=1
    fi
done
test $FOUND == 0
echo -e "\tDone."
