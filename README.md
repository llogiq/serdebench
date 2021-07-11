# Results as of 2021-7-11

```
Benchmarking all/sr.json
Benchmarking all/sr.json: Warming up for 3.0000 s
Benchmarking all/sr.json: Collecting 100 samples in estimated 5.0005 s (40M iterations)
Benchmarking all/sr.json: Analyzing
all/sr.json             time:   [124.99 ns 125.43 ns 125.87 ns]
                        change: [+1.8021% +2.4562% +3.1145%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
json: 100 bytes
Benchmarking all/de.json
Benchmarking all/de.json: Warming up for 3.0000 s
Benchmarking all/de.json: Collecting 100 samples in estimated 5.0002 s (15M iterations)
Benchmarking all/de.json: Analyzing
all/de.json             time:   [324.17 ns 325.17 ns 326.34 ns]
                        change: [-12.162% -10.665% -9.4681%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
Benchmarking all/sr.simd-json
Benchmarking all/sr.simd-json: Warming up for 3.0000 s
Benchmarking all/sr.simd-json: Collecting 100 samples in estimated 5.0000 s (149M iterations)
Benchmarking all/sr.simd-json: Analyzing
all/sr.simd-json        time:   [33.422 ns 33.512 ns 33.612 ns]
                        change: [+1.3854% +1.8840% +2.5161%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  2 (2.00%) high mild
  4 (4.00%) high severe
simd-json: 100 bytes
Benchmarking all/de.simd-json
Benchmarking all/de.simd-json: Warming up for 3.0000 s
Benchmarking all/de.simd-json: Collecting 100 samples in estimated 5.0005 s (13M iterations)
Benchmarking all/de.simd-json: Analyzing
all/de.simd-json        time:   [385.32 ns 388.45 ns 391.81 ns]
                        change: [+1.2330% +2.4929% +3.3764%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
simd-json: 100 bytes
Benchmarking all/sr.yaml
Benchmarking all/sr.yaml: Warming up for 3.0000 s
Benchmarking all/sr.yaml: Collecting 100 samples in estimated 5.0062 s (2.5M iterations)
Benchmarking all/sr.yaml: Analyzing
all/sr.yaml             time:   [2.0130 us 2.0171 us 2.0223 us]
                        change: [-3.0617% -2.4015% -1.8058%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high severe
yaml: 99 bytes
Benchmarking all/de.yaml
Benchmarking all/de.yaml: Warming up for 3.0000 s
Benchmarking all/de.yaml: Collecting 100 samples in estimated 5.0216 s (970k iterations)
Benchmarking all/de.yaml: Analyzing
all/de.yaml             time:   [5.1282 us 5.1354 us 5.1435 us]
                        change: [-1.1016% -0.7279% -0.3964%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
Benchmarking all/sr.ron
Benchmarking all/sr.ron: Warming up for 3.0000 s
Benchmarking all/sr.ron: Collecting 100 samples in estimated 5.0010 s (12M iterations)
Benchmarking all/sr.ron: Analyzing
all/sr.ron              time:   [405.89 ns 407.39 ns 408.89 ns]
                        change: [-4.3377% -3.8998% -3.4951%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
ron: 91 bytes
Benchmarking all/de.ron
Benchmarking all/de.ron: Warming up for 3.0000 s
Benchmarking all/de.ron: Collecting 100 samples in estimated 5.0014 s (6.9M iterations)
Benchmarking all/de.ron: Analyzing
all/de.ron              time:   [716.84 ns 718.28 ns 719.73 ns]
                        change: [-3.4523% -3.1115% -2.7728%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe
Benchmarking all/sr.bincode
Benchmarking all/sr.bincode: Warming up for 3.0000 s
Benchmarking all/sr.bincode: Collecting 100 samples in estimated 5.0001 s (288M iterations)
Benchmarking all/sr.bincode: Analyzing
all/sr.bincode          time:   [17.078 ns 17.152 ns 17.236 ns]
                        change: [-2.2359% -1.3269% -0.4146%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
bincode: 58 bytes
Benchmarking all/de.bincode
Benchmarking all/de.bincode: Warming up for 3.0000 s
Benchmarking all/de.bincode: Collecting 100 samples in estimated 5.0004 s (62M iterations)
Benchmarking all/de.bincode: Analyzing
all/de.bincode          time:   [80.206 ns 80.859 ns 81.664 ns]
                        change: [+1.3173% +2.6368% +4.1181%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  4 (4.00%) high mild
  6 (6.00%) high severe
Benchmarking all/sr.msgpack
Benchmarking all/sr.msgpack: Warming up for 3.0000 s
Benchmarking all/sr.msgpack: Collecting 100 samples in estimated 5.0002 s (98M iterations)
Benchmarking all/sr.msgpack: Analyzing
all/sr.msgpack          time:   [51.719 ns 52.136 ns 52.603 ns]
                        change: [-1.2203% -0.4552% +0.3439%] (p = 0.26 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  3 (3.00%) high mild
  2 (2.00%) high severe
msgpack: 24 bytes
Benchmarking all/de.msgpack
Benchmarking all/de.msgpack: Warming up for 3.0000 s
Benchmarking all/de.msgpack: Collecting 100 samples in estimated 5.0002 s (38M iterations)
Benchmarking all/de.msgpack: Analyzing
all/de.msgpack          time:   [127.56 ns 128.20 ns 128.87 ns]
                        change: [+1.5455% +2.1148% +2.6531%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  6 (6.00%) high mild
  3 (3.00%) high severe
Benchmarking all/sr.cbor
Benchmarking all/sr.cbor: Warming up for 3.0000 s
Benchmarking all/sr.cbor: Collecting 100 samples in estimated 5.0002 s (54M iterations)
Benchmarking all/sr.cbor: Analyzing
all/sr.cbor             time:   [93.924 ns 94.273 ns 94.623 ns]
                        change: [-1.3894% -0.6993% +0.0027%] (p = 0.05 < 0.05)
                        Change within noise threshold.
cbor: 72 bytes
Benchmarking all/de.cbor
Benchmarking all/de.cbor: Warming up for 3.0000 s
Benchmarking all/de.cbor: Collecting 100 samples in estimated 5.0008 s (14M iterations)
Benchmarking all/de.cbor: Analyzing
all/de.cbor             time:   [355.23 ns 356.70 ns 358.27 ns]
                        change: [-0.4447% +0.1738% +0.8787%] (p = 0.62 > 0.05)
                        No change in performance detected.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe
Benchmarking all/sr.postcard
Benchmarking all/sr.postcard: Warming up for 3.0000 s
Benchmarking all/sr.postcard: Collecting 100 samples in estimated 5.0001 s (115M iterations)
Benchmarking all/sr.postcard: Analyzing
all/sr.postcard         time:   [42.587 ns 42.670 ns 42.762 ns]
                        change: [-0.0027% +0.3112% +0.6821%] (p = 0.08 > 0.05)
                        No change in performance detected.
Found 15 outliers among 100 measurements (15.00%)
  7 (7.00%) high mild
  8 (8.00%) high severe
postcard: 41 bytes
Benchmarking all/de.postcard
Benchmarking all/de.postcard: Warming up for 3.0000 s
Benchmarking all/de.postcard: Collecting 100 samples in estimated 5.0006 s (40M iterations)
Benchmarking all/de.postcard: Analyzing
all/de.postcard         time:   [123.60 ns 123.97 ns 124.42 ns]
                        change: [-1.6213% -1.2367% -0.8018%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
Benchmarking all/ser.flexbuffers
Benchmarking all/ser.flexbuffers: Warming up for 3.0000 s
Benchmarking all/ser.flexbuffers: Collecting 100 samples in estimated 5.0012 s (5.0M iterations)
Benchmarking all/ser.flexbuffers: Analyzing
all/ser.flexbuffers     time:   [987.11 ns 990.79 ns 995.13 ns]
                        change: [+0.3723% +0.6089% +0.8631%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe
flexbuffers: 41 bytes
Benchmarking all/de.flexbuffers
Benchmarking all/de.flexbuffers: Warming up for 3.0000 s
Benchmarking all/de.flexbuffers: Collecting 100 samples in estimated 5.0027 s (7.9M iterations)
Benchmarking all/de.flexbuffers: Analyzing
all/de.flexbuffers      time:   [627.30 ns 628.88 ns 630.60 ns]
                        change: [+2.4059% +2.7092% +2.9835%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe
Benchmarking all/sr.flatbuffers
Benchmarking all/sr.flatbuffers: Warming up for 3.0000 s
Benchmarking all/sr.flatbuffers: Collecting 100 samples in estimated 5.0004 s (39M iterations)
Benchmarking all/sr.flatbuffers: Analyzing
all/sr.flatbuffers      time:   [127.64 ns 128.47 ns 129.62 ns]
                        change: [+0.7150% +1.8457% +2.9016%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  4 (4.00%) high severe
flatbuffers: 104 bytes
Benchmarking all/de.flatbuffers
Benchmarking all/de.flatbuffers: Warming up for 3.0000 s
Benchmarking all/de.flatbuffers: Collecting 100 samples in estimated 5.0000 s (73M iterations)
Benchmarking all/de.flatbuffers: Analyzing
all/de.flatbuffers      time:   [68.364 ns 68.751 ns 69.173 ns]
                        change: [-0.7764% +0.1948% +1.1765%] (p = 0.74 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe
Benchmarking all/sr.capnproto.unpacked
Benchmarking all/sr.capnproto.unpacked: Warming up for 3.0000 s
Benchmarking all/sr.capnproto.unpacked: Collecting 100 samples in estimated 5.0002 s (40M iterations)
Benchmarking all/sr.capnproto.unpacked: Analyzing
all/sr.capnproto.unpacked
                        time:   [126.17 ns 126.84 ns 127.69 ns]
                        change: [-1.1325% -0.6392% -0.0332%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
capnproto.unpacked: 96 bytes
Benchmarking all/de.capnproto.unpacked
Benchmarking all/de.capnproto.unpacked: Warming up for 3.0000 s
Benchmarking all/de.capnproto.unpacked: Collecting 100 samples in estimated 5.0010 s (21M iterations)
Benchmarking all/de.capnproto.unpacked: Analyzing
all/de.capnproto.unpacked
                        time:   [245.21 ns 246.21 ns 247.31 ns]
                        change: [+2.5723% +3.1522% +3.7884%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 8 outliers among 100 measurements (8.00%)
  6 (6.00%) high mild
  2 (2.00%) high severe
Benchmarking all/sr.capnproto.packed
Benchmarking all/sr.capnproto.packed: Warming up for 3.0000 s
Benchmarking all/sr.capnproto.packed: Collecting 100 samples in estimated 5.0009 s (25M iterations)
Benchmarking all/sr.capnproto.packed: Analyzing
all/sr.capnproto.packed time:   [199.67 ns 199.87 ns 200.09 ns]
                        change: [-0.4796% -0.1597% +0.1178%] (p = 0.31 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild
capnproto.packed: 39 bytes
Benchmarking all/de.capnproto.packed
Benchmarking all/de.capnproto.packed: Warming up for 3.0000 s
Benchmarking all/de.capnproto.packed: Collecting 100 samples in estimated 5.0011 s (15M iterations)
Benchmarking all/de.capnproto.packed: Analyzing
all/de.capnproto.packed time:   [339.32 ns 339.89 ns 340.52 ns]
                        change: [+3.7820% +3.9796% +4.1660%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
Benchmarking all/sr.proto3
Benchmarking all/sr.proto3: Warming up for 3.0000 s
Benchmarking all/sr.proto3: Collecting 100 samples in estimated 5.0003 s (41M iterations)
Benchmarking all/sr.proto3: Analyzing
all/sr.proto3           time:   [121.63 ns 121.96 ns 122.41 ns]
                        change: [+2.3706% +4.0957% +5.8982%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  10 (10.00%) high severe
proto3: 23 bytes
Benchmarking all/sr.abomonation
Benchmarking all/sr.abomonation: Warming up for 3.0000 s
Benchmarking all/sr.abomonation: Collecting 100 samples in estimated 5.0000 s (701M iterations)
Benchmarking all/sr.abomonation: Analyzing
all/sr.abomonation      time:   [7.0589 ns 7.0663 ns 7.0746 ns]
                        change: [+1.7617% +2.0079% +2.1992%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 5 outliers among 100 measurements (5.00%)
  5 (5.00%) high mild
abomonation: 116 bytes
Benchmarking all/de.abomonation
Benchmarking all/de.abomonation: Warming up for 3.0000 s
Benchmarking all/de.abomonation: Collecting 100 samples in estimated 5.0000 s (557M iterations)
Benchmarking all/de.abomonation: Analyzing
all/de.abomonation      time:   [8.9536 ns 8.9712 ns 8.9975 ns]
                        change: [+0.3751% +0.5607% +0.8147%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe
Benchmarking all/sr.rkyv
Benchmarking all/sr.rkyv: Warming up for 3.0000 s
Benchmarking all/sr.rkyv: Collecting 100 samples in estimated 5.0007 s (24M iterations)
Benchmarking all/sr.rkyv: Analyzing
all/sr.rkyv             time:   [208.21 ns 209.59 ns 211.37 ns]
                        change: [-2.1901% -1.7024% -1.0998%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) high mild
  2 (2.00%) high severe
rkyv: 72 bytes
Benchmarking all/de.rkyv (unvalidated)
Benchmarking all/de.rkyv (unvalidated): Warming up for 3.0000 s
Benchmarking all/de.rkyv (unvalidated): Collecting 100 samples in estimated 5.0000 s (7.0B iterations)
Benchmarking all/de.rkyv (unvalidated): Analyzing
all/de.rkyv (unvalidated)
                        time:   [706.20 ps 711.30 ps 717.64 ps]
                        change: [+1.2710% +2.3833% +3.5379%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 9 outliers among 100 measurements (9.00%)
  2 (2.00%) high mild
  7 (7.00%) high severe
Benchmarking all/de.rkyv (validated)
Benchmarking all/de.rkyv (validated): Warming up for 3.0000 s
Benchmarking all/de.rkyv (validated): Collecting 100 samples in estimated 5.0000 s (229M iterations)
Benchmarking all/de.rkyv (validated): Analyzing
all/de.rkyv (validated) time:   [21.801 ns 21.981 ns 22.199 ns]
                        change: [+3.4805% +4.1815% +4.9266%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

```
