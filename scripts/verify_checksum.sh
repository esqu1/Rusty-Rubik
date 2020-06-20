#!/bin/bash
sha1sum corners.pt edges_o.pt edges_p.pt | diff pruning_tables.checksum -
if [ $? -eq 0 ]
then
    echo "Pruning tables verified!"
else 
    echo "Your pruning tables were generated incorrectly."
fi
