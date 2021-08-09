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

Also as a note, rtl_tcp prints the settings being requested by rtlamr, so we could probably get a raw dump as per the instructions on this link: http://www.aaronscher.com/wireless_com_SDR/RTL_SDR_AM_spectrum_demod.html. Then, we can point rtl_amr at this dump and use the completion time of that as a gauge of efficiency. Also it would be hilarious to benchmark the M1 MacBook Air against the Pi on this same task, hehe. Let's do that next!

Trace from rtl_tcp upon startup:

```
client accepted! localhost 47822
Allocating 15 zero-copy buffers
set freq 912600155
set sample rate 2359296
set gain mode 0
```

So, the capture command could be:

```
rtl_sdr -f 912600155 -s 2359296 -n $(expr 2359296 * 20)
```

For 20 seconds of data.

---

Ok, after a hilarious amount of shenanigans we have the dump, have copied it to the Air (tailscale encryption is rough on the poor Pi) and we have some data coming in.

For the M1:

```
  0.92 real         0.96 user         0.37 sys
      13664256  maximum resident set size
             0  average shared memory size
             0  average unshared data size
             0  average unshared stack size
           906  page reclaims
             0  page faults
             0  swaps
             0  block input operations
             0  block output operations
             3  messages sent
         38465  messages received
            68  signals received
             7  voluntary context switches
        118592  involuntary context switches
   14611620494  instructions retired
    3836224095  cycles elapsed
      11273512  peak memory footprint
```

For the RPi with correct arch:

```
real    1m57.562s
user    1m11.103s
sys     0m12.077s
```

This means that the M1 is (`(60 + 57)/0.92`) 127 times faster than the Pi. Lmfaoooo.

As for the Pi, the one with incorrect arch wouldn't finish on the 300MB input, so a trimmed down 10MB input looks like this:

```
armv5: (incorrect for the hw)
  real    0m27.523s
  user    0m17.421s
  sys     0m0.490s
armv6:
  real    0m9.255s
  user    0m2.268s
  sys     0m0.409s
```

For those following along at home, this means a (`1 - 9.2/27.5`) 66.5% speedup just from using the correct arch!!!!!

---

Ok so can we still optimize more?

~40% of runtime is going to just the `protocol.Decoder.Filter` function, which is fairly understandable and only 10 lines.

```
go tool objdump -S -s protocol.Decoder.Filter rtlamr-armv6
```

We can see that this is probably where all the floating point gains came from. Disassembly of both compiled binaries shows that the v6 one is easily half the size of the v5 one, and doesn't include the `runtime.f*` calls.

Somewhere in the asm is probably the VFPv2 instructions. Apparently they can be used in a vectorized version for multiple registers at a time, but this was "deprecated in favor of NEON".

Also apparently this cpu also supports some sort of `dsp` feature, standing for Digital Signal Processing. Might berelevant.

This page was very useful in decoding the output of `cat /proc/cpuinfo` that this info came from: https://unix.stackexchange.com/a/43563

(Fun write up on the CPU that this RPi uses? https://sandsoftwaresound.net/raspberry-pi/arm11-microarchitecture/ )

## Sun Aug  8 19:52:33 MDT 2021

After great sacrifice I got the CHIP reflashed which seemed to fix the issues. This is about 2 years younger than the R Pi, I believe dating to 2016.

```
root@chip:~# cat /proc/cpuinfo
processor       : 0
model name      : ARMv7 Processor rev 2 (v7l)
BogoMIPS        : 429.72
Features        : half thumb fastmult vfp edsp thumbee neon vfpv3 tls vfpd32
CPU implementer : 0x41
CPU architecture: 7
CPU variant     : 0x3
CPU part        : 0xc08
CPU revision    : 2

Hardware        : Allwinner sun4i/sun5i Families
Revision        : 0000
Serial          : 162542d50041b01b
```

Specs online claim it runs at 1 GHz.

Running the same armv6 binary as the pi (again, the 10mb dataset)

```
armv6 on CHIP:
  real    0m6.374s
  user    0m1.015s
  sys     0m0.150s
```

Looks like a further (`1 - 6.3/9.2`) 31.5% speedup!! I'll note that this is nearly exactly proportional to the differences in clock speed. 🤔😆

Let's compile an armv7 binary and see how that does.

```
armv7 on CHIP:
  real    0m6.452s
  user    0m1.050s
  sys     0m0.145s
```

Nah, negligible improvement. I could take a look at the asm but I'd expect it's the same.

Simply for shits & giggles here is the full 300meg dataset:

```
real    1m53.261s
user    0m33.295s
sys     0m39.055s
```

Interestingly this is only (`1 - (60+53)/(60+57)`) 3.4% faster!! Wonder if there is thermal throttling or something happening. That is just brutal.

WAIT:

Ran it again and got vastly different results:
```
real    0m47.819s
user    0m32.490s
sys     0m3.965s
```

Maybe we were accidentally testing CPU _and_ disk perf last time. Now it's in RAM hopefully. 

Rerunning the 10 MB one as well:

```

real    0m6.428s
user    0m1.115s
sys     0m0.100s
```

Nope, about the same. Interesting. Maybe we're at some sort of IO bottleneck on the smaller dataset??

But the first one is now 59% faster. Weird weird weird. I'll rerun really quickly on the Pi.


### OFFICIAL Benchmarks
```
PI small
real    0m9.057s
user    0m2.283s
sys     0m0.426s

CHIP small
real    0m6.432s
user    0m1.115s
sys     0m0.095s

M1 small
0.06s user 
0.03s system 
150% cpu 
0.061 total

Pi bigg
real    1m52.385s
user    1m12.106s
sys     0m11.456s

CHIP bigg
real    0m45.645s
user    0m32.660s
sys     0m3.280s

M1 bigg
0.97s user
0.37s system
144% cpu 
0.928 total
```

### Times and speeups over Pi

|       | Pi (armv6)  | CHIP (armv7)   | M1 (armv8)    |
| small | - (9.05s)   | 29.0% (6.43s)  | 99.3% (0.06s) |
| large | - (112.39s) | 59.4% (45.65s) | 98.4% (0.93s) |

(All devices using their proper GOARM version.)
