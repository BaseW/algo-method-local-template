if [ $# != 1 ];then
  echo "Please set question number"
  exit 1
fi

questionNo=$1

git add . && git commit -m "answer for question $questionNo"
