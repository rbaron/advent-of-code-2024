
Program: 2,4,1,3,7,5,1,5,0,3,4,3,5,5,3,0

| pos | op  | operand | update                         |
| --- | --- | ------- | ------------------------------ |
| 0   | 2   | 4       | b = a % 8                      |
| 2   | 1   | 3       | b = b ^ 3 = b ^ 0b11           |
| 4   | 7   | 5       | c = a / 2**b                   |
| 6   | 1   | 5       | b = b ^ 5                      |
| 8   | 0   | 3       | a = a / 8                      |
| 10  | 4   | 3       | b = b ^ c                      |
| 12  | 5   | 5       | print(b % 8)                   |
| 14  | 3   | 0       | if a => goto 0; otherwise HALT |


backwards:

- a = 0; b = 0, c = ?
	print(b % 8) => 0
- a = 0; b = 0, c = 0
	b = 0 = b0 ^ c
- a = 0; b = 0, c = 0
	a = a / 8
- a = 0..7, b = 0, c = 0
	b = 0
- a = 0..7, b = ?, c = 0
	c = a / 2 ** b
- a = 0..7, b = log2(a/c), c = ?
	b = b ^ 3 (we can build a table?)
- a = 0..7, b = maybe, c = ?
	b = a % 8 (we don't know prev b, but we know a)


Simplified:

c = a / 2 ** ( (a % 8) ^ 3 )






