#!/usr/bin/env python3
import sys, os
import re
import subprocess
import numpy as np
import pandas as pd
import matplotlib.pyplot as plt

BENCHES = [
    ("read", "Pop Heavy"),
    ("write", "Push Heavy"),
    ("mixed", "Mixed"),
    ("mem", "Memory Heavy")
]

IMPLS = [
    ("mutex", "Mutex"),
    ("spin", "Spin Lock"),
    ("lockfree", "lockfree Library"),
    ("crossbeam", "crossbeam Library"),
    ("dirty", "Lock-Free (no GC)"),
    ("epoch", "Lock-Free (epoch-based GC)"),
]

outdir = "output/"
ratio = 3 / 5

def handle_cli():
    fname = "final_times.csv"

    if len(sys.argv) not in [1, 2]:
        print("Usage ./plot <file (default: times.csv)>")
        sys.exit(1)

    if len(sys.argv) > 1:
        fname = sys.argv[1]

    if not os.path.exists(fname):
        print("Invalid csv file: " + fname)
        sys.exit(1)

    print("Using {} for input".format(fname))
    print("Sending plots to " + outdir)
    os.system("mkdir -p " + outdir)

    return pd.read_csv(fname)

def make_time_vs_cores_plot(data):
    plt.clf()

    # 2 by 2 grid for four benchmarks
    sz = 15
    fig, axs = plt.subplots(2, 2, figsize=(sz, sz*ratio), dpi=300)

    for i, (bench, bench_name) in enumerate(BENCHES):
        # Calculate axis
        x, y = i // 2, i % 2
        ax = axs[x, y]

        for c_i, (impl, impl_name) in enumerate(IMPLS):
            d = data.loc[(data['Benchmark'] == bench)
                & (data["Implementation"] == impl)]

            ax.plot(d["Threads"],
                    d["Time"],
                    label=impl_name,
                    color="C"+str(c_i))

        ax.set_ylabel("Runtime (ms)")
        ax.set_xlabel("Number of Threads")
        ax.set_title(bench_name)

    h, l = axs[0, 0].get_legend_handles_labels()
    fig.legend(h, l, loc="upper right")
    plt.suptitle("Runtimes of Queue Implementations")
    # plt.tight_layout()
    plt.savefig(os.path.join(outdir, "time_vs_cores.png"))

def make_memory_vs_cores_plot(data):
    plt.clf()

    # 2 by 2 grid for four benchmarks
    sz = 12
    fig, ax = plt.subplots(1, 1, figsize=(sz, sz*ratio), dpi=300)

    bench, bench_name = ("mem", "Memory Heavy")
    for c_i, (impl, impl_name) in enumerate(IMPLS):
        d = data.loc[(data['Benchmark'] == bench)
            & (data["Implementation"] == impl)]

        ax.plot(d["Threads"],
                d["Memory"] / (10 ** 3),
                label=impl_name,
                color="C"+str(c_i))

    ax.set_ylabel("Peak Memory Usage (MB)")
    ax.set_xlabel("Number of Threads")
    ax.set_title(bench_name)

    h, l = ax.get_legend_handles_labels()
    fig.legend(h, l, loc="upper right")

    plt.suptitle("Peak Memory Usage of Queue Implementations")
    # plt.tight_layout()
    plt.savefig(os.path.join(outdir, "memory_vs_cores.png"))


if __name__ == '__main__':
    data = handle_cli()

    make_time_vs_cores_plot(data)
    make_memory_vs_cores_plot(data)
