# Log

History of the project, TODOs, etc.

Arranged in descending date order.

## Fri Aug  6 20:43:39 MDT 2021

Thought about doing some sort of "Joy Division" visualization to get more than just the day. Didnt' feel like it.

Instead, performance vis.

https://pkg.go.dev/runtime/pprof

```
➜  power-meters git:(master) ✗ go tool pprof cpu-1.prof
File: rtlamr-prof
Type: cpu
Time: Aug 6, 2021 at 8:34pm (MDT)
Duration: 3.88mins, Total samples = 2.58mins (66.60%)
Entering interactive mode (type "help" for commands, "o" for options)
(pprof) top
Showing nodes accounting for 148.17s, 95.59% of 155s total
Dropped 288 nodes (cum <= 0.78s)
Showing top 10 nodes out of 28
      flat  flat%   sum%        cum   cum%
    65.10s 42.00% 42.00%    120.66s 77.85%  runtime.fadd64
    37.52s 24.21% 66.21%     37.52s 24.21%  runtime.fpack64
    17.97s 11.59% 77.80%     17.97s 11.59%  runtime.funpack64
    14.75s  9.52% 87.32%    111.21s 71.75%  github.com/bemasher/rtlamr/protocol.Decoder.Filter
     5.65s  3.65% 90.96%     29.80s 19.23%  github.com/bemasher/rtlamr/protocol.MagLUT.Execute
     2.28s  1.47% 92.43%      2.28s  1.47%  runtime.futex
     1.99s  1.28% 93.72%      2.39s  1.54%  github.com/bemasher/rtlamr/protocol.(*Decoder).Search
     1.57s  1.01% 94.73%      1.65s  1.06%  syscall.Syscall
     1.18s  0.76% 95.49%      1.18s  0.76%  runtime.memmove
     0.16s   0.1% 95.59%      0.84s  0.54%  runtime.schedule
```

A vast majority spent in floating point ops? Interesting. Very very little spent in Search, which based on my very cursory read through the code the other day isn't at all what I would have expected.

Q: Does the R Pi have floating point support?
A: Seems to. With help from https://unix.stackexchange.com/questions/144806/how-can-i-tell-if-floating-point-arithmetic-is-performed-in-hardware-or-software we see:

```
pi@raspberrypi:~ $ grep Features /proc/cpuinfo
Features        : half thumb fastmult vfp edsp java tls
```

Hmmm did I compile for wrong version??????

```
model name      : ARMv6-compatible processor rev 7 (v6l)
```

Compiled a version for GOARM=6 instead of GOARM=5. Binary sizes are the same.... but also maybe they're just nearly the same who knows. Doing more profiling...

```
➜  rtlamr git:(master) ✗ go tool pprof cpu.prof
File: rtlamr-armv6-prof
Type: cpu
Time: Aug 6, 2021 at 8:57pm (MDT)
Duration: 4.01mins, Total samples = 2.08mins (51.91%)
Entering interactive mode (type "help" for commands, "o" for options)
(pprof) top
Showing nodes accounting for 1.64mins, 78.80% of 2.08mins total
Dropped 343 nodes (cum <= 0.01mins)
Showing top 10 nodes out of 61
      flat  flat%   sum%        cum   cum%
  0.78mins 37.35% 37.35%   0.80mins 38.24%  github.com/bemasher/rtlamr/protocol.Decoder.Filter
  0.32mins 15.53% 52.87%   0.32mins 15.53%  github.com/bemasher/rtlamr/protocol.MagLUT.Execute
  0.16mins  7.73% 60.60%   0.20mins  9.59%  github.com/bemasher/rtlamr/protocol.(*Decoder).Search
  0.14mins  6.56% 67.16%   0.15mins  7.02%  syscall.Syscall
  0.10mins  4.93% 72.09%   0.10mins  4.93%  runtime.memmove
  0.04mins  1.84% 73.93%   0.04mins  1.84%  github.com/bemasher/rtlamr/protocol.searchPassByte
  0.04mins  1.69% 75.62%   0.04mins  1.99%  runtime.checkTimers
  0.03mins  1.62% 77.24%   0.03mins  1.62%  runtime._LostSIGPROFDuringAtomic64
  0.02mins  0.89% 78.13%   0.02mins  0.89%  math.Float64bits
  0.01mins  0.67% 78.80%   0.01mins  0.67%  runtime.duffcopy
```

HA. Omg. So allegedly we're going much faster now? Very interesting.

Also interesting that CPU is still maxed out on the Pi. It's probably just going as fast as it can, it can just go much faster now.

(Also as a note, rtl_tcp prints the settings being requested by rtlamr, so we could probably get a raw dump as per the instructions on this link: http://www.aaronscher.com/wireless_com_SDR/RTL_SDR_AM_spectrum_demod.html. Then, we can point rtl_amr at this dump and use the completion time of that as a gauge of efficiency. (Also also it would be hilarious to benchmark the M1 MacBook Air against the Pi on this same task, hehe.))
