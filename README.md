# pipelines - experiments in shuffling data

This repository contains some experiments in shuffling data through a straight
pipeline, from stdin through a transform and then out on stdout. Here's an
example:

```
> echo "hello world" | ./target/release/std-baseline
[spaces: 1]hello world
```

Our goal is to achieve the highest throughput possible. You can get a sense of
this by running [GNU
yes](https://www.reddit.com/r/unix/comments/6gxduc/how_is_gnu_yes_so_fast/)
through pv. Here's the top-line for my system:

```
> yes "abcdefg" | pv --average-rate > /dev/null
[8.55GiB/s]
```

## Summary

Every program in this repository has sharply less throughput than theoretical
maximum. Each program is _line oriented_, meaning it operates on the level of a
single line traveling through the system. I conjecture that overhead per line
dominates program runtime. `std-baseline` reduces this overhead but serializes
lines read from stdin with writes to stdout. Overall throughput is 1/10th of
theoretical maximum. The second-best performer, `streamer` is similar to
`baseline` except in that it reduces overhead by offering a somewhat better
optimization target for Rust. However, for this program at least, this comes at
an increased cost in type complexity: when the type checker complains about code
in `streamer` it REALLY complains. Repeat `perf` runs also show that, for these
programs, the runtime of `baseline`, `streamer` and `std-baseline` are roughly
the same, though the variance of `baseline` is significantly worse. In the
`perf` runs `pipeline` is consistently two seconds slower than its competitors
on my system; its variance is somewhat less worse than `baseline`. Not only does
`pipeline` have per-line overhead but it adds MPSC and task switching costs not
present in the other programs.

## Future Work

I intend to validate my hypothesis that the major cost of these programs is the
per-line approach taken. If we batch inputs internally, assuming the hypothesis
is correct, we should suffer less cache invalidation in our running programs
_and_ be able to exploit multiple CPU cores. This will increase per-line latency
but, assuming the hypothesis is correct, improve overall throughput.

## Background

There are four programs:

* `std-baseline`: uses sync code from std in one loop
* `baseline`: uses tokio and a single task
* `streamer`: uses tokio/futures-streams in one task
* `pipeline`: uses tokio with multiple tasks communicating across MPSC queues

Each program is intended to do the same thing but help us explore different
approaches to achieving it. Aside from buffering at the stdin boundary there is
no internal batching. Backpressure is accomplised in `pipeline` by using bounded
queues, in the other programs by maintaining sequential/async program
structure. Here's their rough throughput on my system:

```
> ./throughput.sh
    Finished release [optimized + debuginfo] target(s) in 0.02s
std-baseline: [ 811MiB/s]
baseline: [ 367MiB/s]
pipeline: [ 358MiB/s]
streamer: [ 405MiB/s]
```

Your numbers will vary. For _this_ particular program where there is very little
CPU computation and a simple, straight IO pipeline the overhead inherent to a
multi-threaded work stealing scheduler is apparent.

If you would like to generate `perf` data run `./stress.sh`. If you would like
to get a quick benchmark run `./stress_one.sh`. Here's a run of `./stress.sh` on
my system, with some detail removed for clarity:

```
> ./stress.sh
 Performance counter stats for 'bash -c zcat /home/blt/projects/us/troutwine/pipeline/resources/stable_big.log.gz | /home/blt/projects/us/troutwine/pipeline/target/release/baseline > /dev/null' (5 runs):

         11,531.11 msec task-clock                #    2.316 CPUs utilized            ( +-  3.47% )
           554,008      context-switches          #    0.048 M/sec                    ( +-  1.09% )
               338      cpu-migrations            #    0.029 K/sec                    ( +- 13.50% )
               898      page-faults               #    0.078 K/sec                    ( +-  0.06% )
    36,188,505,840      cycles                    #    3.138 GHz                      ( +-  0.45% )  (83.55%)
     1,733,695,725      stalled-cycles-frontend   #    4.79% frontend cycles idle     ( +-  3.91% )  (83.24%)
    14,523,012,533      stalled-cycles-backend    #   40.13% backend cycles idle      ( +-  2.62% )  (83.28%)
    52,860,811,364      instructions              #    1.46  insn per cycle
                                                  #    0.27  stalled cycles per insn  ( +-  0.14% )  (83.31%)
     9,370,980,070      branches                  #  812.670 M/sec                    ( +-  0.26% )  (83.44%)
       174,330,612      branch-misses             #    1.86% of all branches          ( +-  0.46% )  (83.19%)

             4.978 +- 0.296 seconds time elapsed  ( +-  5.95% )

 Performance counter stats for 'bash -c zcat /home/blt/projects/us/troutwine/pipeline/resources/stable_big.log.gz | /home/blt/projects/us/troutwine/pipeline/target/release/std-baseline > /dev/null' (5 runs):

          5,885.51 msec task-clock                #    1.429 CPUs utilized            ( +-  1.16% )
            31,849      context-switches          #    0.005 M/sec                    ( +-  0.01% )
                22      cpu-migrations            #    0.004 K/sec                    ( +- 20.78% )
               724      page-faults               #    0.123 K/sec                    ( +-  0.25% )
    23,579,159,699      cycles                    #    4.006 GHz                      ( +-  0.34% )  (83.25%)
       434,439,929      stalled-cycles-frontend   #    1.84% frontend cycles idle     ( +-  0.52% )  (83.58%)
     8,351,341,410      stalled-cycles-backend    #   35.42% backend cycles idle      ( +-  0.32% )  (83.22%)
    43,802,962,408      instructions              #    1.86  insn per cycle
                                                  #    0.19  stalled cycles per insn  ( +-  0.05% )  (83.45%)
     7,661,995,659      branches                  # 1301.841 M/sec                    ( +-  0.02% )  (83.30%)
       165,785,064      branch-misses             #    2.16% of all branches          ( +-  0.09% )  (83.19%)

            4.1175 +- 0.0104 seconds time elapsed  ( +-  0.25% )

 Performance counter stats for 'bash -c zcat /home/blt/projects/us/troutwine/pipeline/resources/stable_big.log.gz | /home/blt/projects/us/troutwine/pipeline/target/release/pipeline > /dev/null' (5 runs):

         17,964.92 msec task-clock                #    2.839 CPUs utilized            ( +-  1.39% )
           573,339      context-switches          #    0.032 M/sec                    ( +-  1.18% )
               497      cpu-migrations            #    0.028 K/sec                    ( +- 16.61% )
             1,065      page-faults               #    0.059 K/sec                    ( +-  0.95% )
    45,719,918,534      cycles                    #    2.545 GHz                      ( +-  0.96% )  (83.27%)
     1,719,547,594      stalled-cycles-frontend   #    3.76% frontend cycles idle     ( +-  1.89% )  (83.29%)
    19,221,301,471      stalled-cycles-backend    #   42.04% backend cycles idle      ( +-  1.80% )  (83.54%)
    67,142,993,939      instructions              #    1.47  insn per cycle
                                                  #    0.29  stalled cycles per insn  ( +-  0.20% )  (83.28%)
    12,283,973,183      branches                  #  683.776 M/sec                    ( +-  0.10% )  (83.74%)
       185,229,679      branch-misses             #    1.51% of all branches          ( +-  0.28% )  (82.87%)

             6.328 +- 0.111 seconds time elapsed  ( +-  1.76% )

 Performance counter stats for 'bash -c zcat /home/blt/projects/us/troutwine/pipeline/resources/stable_big.log.gz | /home/blt/projects/us/troutwine/pipeline/target/release/streamer > /dev/null' (5 runs):

         11,250.04 msec task-clock                #    2.665 CPUs utilized            ( +-  0.78% )
           307,105      context-switches          #    0.027 M/sec                    ( +-  1.26% )
               343      cpu-migrations            #    0.031 K/sec                    ( +- 31.54% )
               942      page-faults               #    0.084 K/sec                    ( +-  0.45% )
    39,190,397,164      cycles                    #    3.484 GHz                      ( +-  0.39% )  (83.33%)
     1,291,706,592      stalled-cycles-frontend   #    3.30% frontend cycles idle     ( +-  2.25% )  (83.28%)
    17,951,796,831      stalled-cycles-backend    #   45.81% backend cycles idle      ( +-  0.87% )  (83.36%)
    69,437,903,454      instructions              #    1.77  insn per cycle
                                                  #    0.26  stalled cycles per insn  ( +-  0.26% )  (83.56%)
    12,574,315,772      branches                  # 1117.713 M/sec                    ( +-  0.22% )  (83.21%)
       184,980,665      branch-misses             #    1.47% of all branches          ( +-  0.23% )  (83.25%)

            4.2215 +- 0.0111 seconds time elapsed  ( +-  0.26% )
```

Because only a single run is done in `stress_one.sh` the numbers are not be very
stable. `stress.sh` controls for this somewhat by running each example 10 times.
