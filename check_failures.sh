for d in `find ./questions -type d -maxdepth 1 -mindepth 1`;
do
  is_rs_found=`find $d -name "*.rs" | wc -l`
  is_py_found=`find $d -name "*.py" | wc -l`
  # echo $d $is_rs_found $is_py_found
  if [ $is_rs_found -eq 1 ] && [ $is_py_found -eq 0 ];
  then
    # echo "Only Rust found in $d"
    echo $d
  fi
done
