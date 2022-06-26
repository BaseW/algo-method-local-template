if [ $# != 1 ];then
  echo "Please set question number"
  exit 1
fi

questionNo=$1
make get_question No=$questionNo || mkdir -p questions/$questionNo
code questions/$questionNo/main.rs

