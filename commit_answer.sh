questionNo=`cat currentNo.txt | tr -d "\n"`

rm currentNo.txt
git add . && git commit -m "answer for question $questionNo"
