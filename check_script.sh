if [ $# != 1 ];then
  echo "Please set question number"
  exit 1
fi

questionNo=$1
rustc questions/$questionNo/main.rs
./main
rm ./main
