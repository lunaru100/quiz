#!/bin/bash

cat $1 | while read line
do
    uuid=$(uuidgen | sed 's/-//g')
    line=$(echo $line | sed "s/id/x'$uuid'/")

    sqlite3 dev.sqlite "$line";

done

