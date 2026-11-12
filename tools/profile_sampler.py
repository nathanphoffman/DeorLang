"""Sampling profiler for the transpiler, run inside gdb (see `just profile`).

perf needs root on many machines, so this stops the program every few
milliseconds with SIGINT and records where it is. It needs no root and no
special build; the binary only has to keep its function names.

Environment:
  PROFILE_ARGS      arguments to run the program with (required)
  PROFILE_OUT       file to write the report to (required)
  PROFILE_INTERVAL  seconds between samples (default 0.002)
"""

import collections
import os
import re
import signal
import threading
import time

import gdb  # provided by gdb itself

START_DELAY_SECONDS = 0.03
MAX_STACK_DEPTH = 80
MAIN_FRAME_PATTERN = re.compile(r"^(\w+)::main$")
SAMPLE_INTERVAL = float(os.environ.get("PROFILE_INTERVAL", "0.002"))
self_counts = collections.Counter()
inclusive_counts = collections.Counter()
nearest_counts = collections.Counter()
list_clone_counts = collections.Counter()
sample_total = 0
crate_name = None  # learned from the first stack that reaches <crate>::main
finished = False


def demangle(mangled):
    """Turns a Rust v0 mangled name into crate::function (keeps the last names)."""
    match = re.match(r"^_R(?:Nv|NC|Nt)*(?:C|M)?s?[0-9A-Za-z]*_(.*)$", mangled)

    if not match:
        return mangled

    rest, parts = match.group(1), []

    # Each piece is <decimal length><name>, e.g. 8tokenize
    while re.match(r"^\d+", rest):
        length_text = re.match(r"^\d+", rest).group(0)
        start = len(length_text)
        parts.append(rest[start : start + int(length_text)])
        rest = rest[start + int(length_text) :]

    return "::".join(parts) if parts else mangled


def stack_names():
    names, frame = [], gdb.newest_frame()

    while frame is not None and len(names) < MAX_STACK_DEPTH:
        names.append(demangle(frame.name() or "??"))
        frame = frame.older()

    return names


def find_crate_name(names):
    for name in names:
        match = MAIN_FRAME_PATTERN.match(name)

        if match:
            return match.group(1)

    return None


def is_list_clone(name):
    is_vec_clone = "as core::clone::Clone" in name and "Vec<" in name

    return is_vec_clone and ("alloc::string::String" in name or "::Token" in name)


def take_sample():
    global sample_total, crate_name
    names = stack_names()
    crate_name = crate_name or find_crate_name(names)

    if crate_name is None:
        return

    own_prefix = crate_name + "::"
    own_names = [
        name for name in names if name.startswith(own_prefix) and "closure" not in name
    ]

    nearest = own_names[0] if own_names else "(runtime)"
    self_counts[names[0]] += 1
    nearest_counts[nearest] += 1
    sample_total += 1

    for name in set(names):
        inclusive_counts[name] += 1

    if any(is_list_clone(name) for name in names):
        list_clone_counts[nearest] += 1


def interrupt_periodically():
    time.sleep(START_DELAY_SECONDS)

    while not finished:
        pid = gdb.selected_inferior().pid

        if pid:
            os.kill(pid, signal.SIGINT)

        time.sleep(SAMPLE_INTERVAL)


def write_section(report, title, counter, limit):
    report.write("--- %s\n" % title)

    for name, count in counter.most_common(limit):
        report.write("%5.1f%% %s\n" % (100 * count / sample_total, name))


def write_report(path):
    with open(path, "w") as report:
        report.write("samples %d\n" % sample_total)
        write_section(
            report,
            "where the time is spent (nearest transpiler function)",
            nearest_counts,
            25,
        )

        write_section(report, "whole-list clones, by caller", list_clone_counts, 15)
        write_section(report, "self time (what the CPU was executing)", self_counts, 20)


threading.Thread(target=interrupt_periodically, daemon=True).start()
gdb.execute("run " + os.environ["PROFILE_ARGS"])

while gdb.selected_inferior().pid:
    take_sample()

    try:
        gdb.execute("continue")
    except gdb.error:
        break

finished = True
write_report(os.environ["PROFILE_OUT"])
