# 概要
在宅状況を返してくれる君 with Rust on Cloudflare Workers

# 内容
家のAPのsyslogをraspberry pi 4Bで監視して、スマホがwifiに接続されたらこのworkerにpost requestを飛ばすようにする。
このworkerはその情報をKVに保存しておく、get requestを受け取ると、KVに保存されている在宅状況を返す。

# 参考
- https://zenn.dev/tfutada/articles/f6be3049d5f7fe
- https://github.com/cloudflare/workers-rs

