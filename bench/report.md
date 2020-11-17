# Benchmark
* stubr
  * rust version: 1.47.0 
* wiremock
  * java version: openjdk 11.0.7 2020-04-14
  * wiremock version: 2.27.2


|  scenario (duration / users) | avg latency (+/-) | avg req/sec (+/-) | total req | total bytes |
|:--------------------------:|:-----------------:|:-----------------:|:---------:|:-----------:|
| stubr-ping (60s / 1) | 69 µs (+/- 8 µs) | 14129 (+/- 335) | 844780 | 63358500 | 
| wiremock-ping (60s / 1) | 116 µs (+/- 1674 µs) | 17206 (+/- 2822) | 1027337 | 141772506 | 
| stubr-ping (60s / 10) | 138 µs (+/- 93 µs) | 7129 (+/- 301) | 4263435 | 319757625 | 
| wiremock-ping (60s / 10) | 357 µs (+/- 2911 µs) | 5018 (+/- 837) | 2991773 | 412864940 | 
| stubr-ping (60s / 100) | 1077 µs (+/- 356 µs) | 936 (+/- 76) | 5593174 | 419488050 | 
| wiremock-ping (60s / 100) | 3709 µs (+/- 20627 µs) | 488 (+/- 78) | 2898758 | 400033658 | 
| stubr-ping (60s / 200) | 2349 µs (+/- 422 µs) | 427 (+/- 32) | 5110623 | 383296725 | 
| wiremock-ping (60s / 200) | 5470 µs (+/- 14070 µs) | 230 (+/- 39) | 2746786 | 379064581 | 