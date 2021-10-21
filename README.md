# WEB+DB PRESS Vol 122 作って学ぶRDBMSのしくみ
## ヒープファイル
ディスクマネージャが読み書きするファイルはヒープファイルと呼ばれる構造をしている。ヒープファイルとはファイルをページと呼ばれる固定の長さに区切ったもの。それぞれのページにはページIDがある。ディスクマネージャはヒープファイルからページへ読み書きする。ページIDを用いて、どこに書き出すか、どこから読み出すかを指定する。1つのページは4096バイトの整数倍のサイズ(1-4倍)。たとえ100バイトの情報だけ必要だったとしても必ずこのサイズ(4096バイト)でしかやりとりができない。 このようにしているのは、ファイルシステムがファイルへの読み書きをブロックサイズ(Linuxの有名なファイルシステムであるext4のデフォルトブロッサイズは4096バイト)単位で行うから。ファイルシステムでの不利な切り上げが起きないように、4096バイトの整数倍にページサイズは指定されている。

## Rustに関する知識
### New Typeパターン
説明を追加する。新たに構造体を定義して、その型の値の演算を間違って行ってしまうのを防ぐ方法(?)

## 参考・引用
- [relly](https://github.com/KOBA789/relly/tree/wdb/src)
