PACKAGE_NAME=(`awk -F ' = ' '$1 ~ /name/ { if(count<1){ gsub(/[\"]/, "", $2); printf("%s",$2) } count++}' ./$1/Cargo.toml`)
echo $PACKAGE_NAME