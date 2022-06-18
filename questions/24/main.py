"""
https://algo-method.com/tasks/24

### 問題文
$2$ つの正の整数 $A, B$ が空白区切りで入力されます。
$A + B$ の値を出力してください。
### 入力
入力は次の形式で与えられます。
```IOExample
$A B$
```
また、入力される値は次の制約を満たします。
- $A, B$ は $1$ 以上 $100$ 以下の整数
### 出力
答えを出力してください。
"""


def main():
    a, b = map(int, input().split())
    print(a + b)


main()
