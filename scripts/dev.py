#!/usr/bin/env python3
"""
Development server runner that manages multiple processes concurrently.
Handles graceful shutdown on Ctrl+C and provides color-coded output.
Automatically shuts down all processes if any process fails.
"""

import subprocess
import sys
import signal
import threading
import os
from pathlib import Path

# ANSI color codes for output
COLORS = {
    'api': '\033[94m',        # Blue
    'web': '\033[92m',        # Green
    'frontend': '\033[92m',   # Green
    'reset': '\033[0m',       # Reset
    'bold': '\033[1m',        # Bold
    'error': '\033[91m',      # Red
}

# Process configurations
PROCESSES = {
    'api': {
        'cmd': ['watchexec', '-e', 'rs,toml', '-r', '--', 'sh', '-c', '"npm run api:generate"'],
        'cwd': '.',
        'color': COLORS['api'],
    },
    'frontend': {
        'cmd': ['npm', 'run', 'dev'],
        'cwd': '.',
        'color': COLORS['web'],
    },
}

# Global list to track running processes
processes = []
shutdown_event = threading.Event()
error_occurred = threading.Event()


def print_colored(name, line, color):
    """Print output with color coding and process name prefix."""
    prefix = f"{color}{COLORS['bold']}[{name}]{COLORS['reset']} "
    print(f"{prefix}{line}", flush=True)


def stream_output(process, name, color):
    """Read and print process output line by line with color coding."""
    try:
        for line in iter(process.stdout.readline, ''):
            if shutdown_event.is_set():
                break
            if line:
                print_colored(name, line.rstrip(), color)
    except ValueError:
        # Process stdout closed
        pass


def shutdown_all_processes(reason="Unknown"):
    """Shutdown all running processes."""
    if shutdown_event.is_set():
        return  # Already shutting down

    print(f"\n{COLORS['bold']}{COLORS['error']}Shutdown initiated: {reason}{COLORS['reset']}")
    shutdown_event.set()

    for name, process in processes:
        if process.poll() is None:  # Process still running
            try:
                print_colored(name, "Terminating...", COLORS['reset'])
                process.terminate()
            except Exception as e:
                print_colored(name, f"Error terminating: {e}", COLORS['reset'])

    # Wait for processes to terminate
    for name, process in processes:
        if process.poll() is None:
            try:
                process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                print_colored(name, "Force killing...", COLORS['reset'])
                process.kill()

    print(f"{COLORS['bold']}All processes stopped.{COLORS['reset']}")


def run_process(name, config):
    """Start a process and stream its output."""
    cwd = Path(__file__).parent.parent / config['cwd']

    print_colored(
        name,
        f"Starting in {config['cwd']}...",
        config['color']
    )

    try:
        process = subprocess.Popen(
            config['cmd'],
            cwd=str(cwd),
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            bufsize=1,
            universal_newlines=True,
        )

        processes.append((name, process))

        # Stream output in the main thread for this process
        stream_output(process, name, config['color'])

        # Wait for process to complete
        returncode = process.wait()

        # If process exited with error and we're not already shutting down
        if returncode != 0 and not shutdown_event.is_set():
            print_colored(
                name,
                f"Process exited with code {returncode}",
                COLORS['error']
            )
            error_occurred.set()
            shutdown_all_processes(f"{name} process failed with exit code {returncode}")

    except FileNotFoundError:
        print_colored(
            name,
            f"ERROR: Command not found: {config['cmd'][0]}",
            COLORS['error']
        )
        error_occurred.set()
        shutdown_all_processes(f"{name} command not found")
    except Exception as e:
        print_colored(
            name,
            f"ERROR: {e}",
            COLORS['error']
        )
        error_occurred.set()
        shutdown_all_processes(f"{name} encountered an error")


def signal_handler(sig, frame):
    """Handle Ctrl+C gracefully."""
    shutdown_all_processes("User interrupt (Ctrl+C)")
    sys.exit(0)


def main():
    """Main entry point."""
    # Register signal handler for Ctrl+C
    signal.signal(signal.SIGINT, signal_handler)

    print(f"{COLORS['bold']}Starting development servers...{COLORS['reset']}\n")

    # Create threads for each process
    threads = []
    for name, config in PROCESSES.items():
        thread = threading.Thread(target=run_process, args=(name, config))
        thread.daemon = True
        thread.start()
        threads.append(thread)

    # Wait for all threads to complete
    try:
        for thread in threads:
            thread.join()

        # If we reach here, check if there was an error
        if error_occurred.is_set():
            sys.exit(1)

    except KeyboardInterrupt:
        signal_handler(signal.SIGINT, None)


if __name__ == '__main__':
    main()
