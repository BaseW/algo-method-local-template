"""
https://algo-method.com/tasks/25

### 問題文
$3$ つの整数 $A, B, C$ が空白区切りで入力されます。$3$ つの整数の平均値を**整数で**出力してください。
ただし、答えは整数になることが保証されています。
### 入力
入力は次の形式で与えられます。
```IOExample
$A B C$
```
また、入力される値は次の制約を満たします。
- $A, B, C$ は $1$ 以上 $100$ 以下の整数
- 答えは整数になることが保証されている。
### 出力
答えを出力してください。
"""


def main():
    a, b, c = map(int, input().split())
    print((a + b + c) / 3)


if __name__ == '__main__':
    main()
