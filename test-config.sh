#!/bin/bash

# Test script for CONFIG command functionality
echo "Testing CONFIG command functionality..."

# Test CONFIG LIST
echo -e "\n1. Testing CONFIG LIST:"
echo -e "*2\r\n\$6\r\nconfig\r\n\$4\r\nlist\r\n" | nc -w 1 127.0.0.1 6379

# Test CONFIG GET gc.interval
echo -e "\n2. Testing CONFIG GET gc.interval:"
echo -e "*3\r\n\$6\r\nconfig\r\n\$3\r\nget\r\n\$11\r\ngc.interval\r\n" | nc -w 1 127.0.0.1 6379

# Test CONFIG GET gc.batch
echo -e "\n3. Testing CONFIG GET gc.batch:"
echo -e "*3\r\n\$6\r\nconfig\r\n\$3\r\nget\r\n\$8\r\ngc.batch\r\n" | nc -w 1 127.0.0.1 6379

# Test CONFIG GET gc.enabled
echo -e "\n4. Testing CONFIG GET gc.enabled:"
echo -e "*3\r\n\$6\r\nconfig\r\n\$3\r\nget\r\n\$10\r\ngc.enabled\r\n" | nc -w 1 127.0.0.1 6379

# Test CONFIG SET gc.interval (valid value)
echo -e "\n5. Testing CONFIG SET gc.interval 500:"
echo -e "*4\r\n\$6\r\nconfig\r\n\$3\r\nset\r\n\$11\r\ngc.interval\r\n\$3\r\n500\r\n" | nc -w 1 127.0.0.1 6379

# Test CONFIG SET gc.batch (valid value)
echo -e "\n6. Testing CONFIG SET gc.batch 200:"
echo -e "*4\r\n\$6\r\nconfig\r\n\$3\r\nset\r\n\$8\r\ngc.batch\r\n\$3\r\n200\r\n" | nc -w 1 127.0.0.1 6379

# Test CONFIG SET gc.enabled (valid value)
echo -e "\n7. Testing CONFIG SET gc.enabled true:"
echo -e "*4\r\n\$6\r\nconfig\r\n\$3\r\nset\r\n\$10\r\ngc.enabled\r\n\$4\r\ntrue\r\n" | nc -w 1 127.0.0.1 6379

# Test CONFIG SET with invalid value
echo -e "\n8. Testing CONFIG SET gc.interval 50 (invalid - too low):"
echo -e "*4\r\n\$6\r\nconfig\r\n\$3\r\nset\r\n\$11\r\ngc.interval\r\n\$2\r\n50\r\n" | nc -w 1 127.0.0.1 6379

# Test CONFIG SET with invalid parameter
echo -e "\n9. Testing CONFIG SET invalid.param value:"
echo -e "*4\r\n\$6\r\nconfig\r\n\$3\r\nset\r\n\$13\r\ninvalid.param\r\n\$5\r\nvalue\r\n" | nc -w 1 127.0.0.1 6379

echo -e "\nCONFIG command testing completed!" 