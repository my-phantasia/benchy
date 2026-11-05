#!/bin/bash
set -euo pipefail

TARGET_CPU=3
FREQ_KHZ=2500000 # 2.5 GHz in KHz for sysfs

ORIG_GOV=$(cat /sys/devices/system/cpu/cpu${TARGET_CPU}/cpufreq/scaling_governor 2>/dev/null || echo "powersave")
ORIG_TURBO=$(cat /sys/devices/system/cpu/intel_pstate/no_turbo 2>/dev/null || echo "0")
ORIG_MIN=$(cat /sys/devices/system/cpu/cpu${TARGET_CPU}/cpufreq/scaling_min_freq 2>/dev/null || echo "$FREQ_KHZ")
ORIG_MAX=$(cat /sys/devices/system/cpu/cpu${TARGET_CPU}/cpufreq/scaling_max_freq 2>/dev/null || echo "$FREQ_KHZ")

cleanup() {
  echo -e "\n==> Restoring original CPU settings..."
  echo "$ORIG_TURBO" | sudo tee /sys/devices/system/cpu/intel_pstate/no_turbo > /dev/null
  echo "$ORIG_MIN" | sudo tee /sys/devices/system/cpu/cpu${TARGET_CPU}/cpufreq/scaling_min_freq > /dev/null
  echo "$ORIG_MAX" | sudo tee /sys/devices/system/cpu/cpu${TARGET_CPU}/cpufreq/scaling_max_freq > /dev/null
  echo "$ORIG_GOV" | sudo tee /sys/devices/system/cpu/cpu${TARGET_CPU}/cpufreq/scaling_governor > /dev/null

  if [ -d "target" ]; then
      sudo chown -R "$USER:$USER" target/ 2>/dev/null || true
  fi

  echo "==> Done. System restored to normal state."
}
trap cleanup EXIT INT TERM

if [ "$#" -eq 0 ]; then
    echo "Error: No benchmark command provided."
    echo "Usage: ./run_bench.sh cargo bench"
    exit 1
fi

echo "==> Tuning system for benchmark..."
echo 1 | sudo tee /sys/devices/system/cpu/intel_pstate/no_turbo > /dev/null
echo "performance" | sudo tee /sys/devices/system/cpu/cpu${TARGET_CPU}/cpufreq/scaling_governor > /dev/null
echo "$FREQ_KHZ" | sudo tee /sys/devices/system/cpu/cpu${TARGET_CPU}/cpufreq/scaling_min_freq > /dev/null || true
echo "$FREQ_KHZ" | sudo tee /sys/devices/system/cpu/cpu${TARGET_CPU}/cpufreq/scaling_max_freq > /dev/null || true

echo "==> Running Rust benchmark on CPU $TARGET_CPU (ASLR disabled, RT priority)..."

sudo env PATH="$PATH" HOME="$HOME" chrt -f 99 taskset -c "$TARGET_CPU" setarch "$(uname -m)" -R "$@"
