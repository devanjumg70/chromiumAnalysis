# Device Emulation Data Investigation Report

## Executive Summary
*   **Goal**: Locate and extract device definition data (names, viewports, user agents) for Rust porting.
*   **Primary Source**: `third_party/devtools-frontend/src/front_end/models/emulation/EmulatedDevices.ts` (TypeScript source).
*   **Secondary Source**: `chrome/test/chromedriver/chrome/mobile_device_list.cc` (Generated C++).
*   **Status**: Primary source is inaccessible due to submodule restrictions. Secondary source (generated file) is missing from the build output.
*   **Workaround**: Reverse-engineered the data schema from `chromedriver` test code and `mobile_device.cc`.

## Investigation Logs

### 1. Initial Search
*   **Target**: `browser_protocol.pdl`, `EmulatedDevices`, or any "device" list.
*   **Tools**: `fdfind` (failed), `find`, `grep`.
*   **Finding**: Located `browser_protocol.pdl` at `third_party/blink/public/devtools_protocol/browser_protocol.pdl`. This defined the *protocol* but not the *data*.

### 2. The `devtools-frontend` Dead End
*   **Attempt**: Search for "EmulatedDevice" or "Nexus 5" in `third_party/devtools-frontend`.
*   **Result**: Empty results. Directory listings showed `third_party/devtools-frontend` exists but `src` subdirectory was empty/inaccessible.
*   **Root Cause**: Verified via `.gitmodules`:
    ```ini
    [submodule "third_party/devtools-frontend/src"]
    	path = third_party/devtools-frontend/src
    	url = https://chromium.googlesource.com/devtools/devtools-frontend
    ```
    The `devtools-frontend` source code is a git submodule that was not fully initialized or accessible in this workspace environment.

### 3. Alternative Discovery: ChromeDriver
*   **Attempt**: Repo-wide search for "Nexus 5" (a known emulated device).
*   **Result**: Found references in `chrome/test/chromedriver`.
    *   `chrome/test/chromedriver/chrome/mobile_device.cc`: logic for parsing device lists.
    *   `chrome/test/chromedriver/embed_mobile_devices_in_cpp.py`: script that *generates* the C++ list from the `devtools-frontend` source.

### 4. Schema Extraction
Since I could not read the raw JSON/TypeScript source, I analyzed:
1.  **`mobile_device.cc`**: The `MobileDevice::FindMobileDevice` method reveals the expected JSON structure (fields: `userAgent`, `deviceMetrics`, `clientHints`).
2.  **`embed_mobile_devices_in_cpp.py`**: The script logic shows how it transforms the TS fields into the C++ embedded JSON.

## Extracted Schema (Rust)
Based on the C++ parsing logic, I defined the following Rust structures in `chromium_analysis/device_struct.rs`:

```rust
struct MobileDevice {
    title: String,
    user_agent: String,
    device_type: String,
    device_metrics: DeviceMetrics, // width, height, scale, etc.
    client_hints: Option<ClientHints>, // arch, platform, model, etc.
}
```

## Next Steps
To get the *full* official list of devices:
1.  **Option A**: Initialize submodules (`git submodule update --init --recursive`) - *Not permitted in this environment*.
2.  **Option B**: Manually fetch the `EmulatedDevices.ts` file from the remote repo URL found in `.gitmodules`.
3.  **Option C**: Use the sample data provided in `chromium_analysis/devices.json` and expand it manually as needed.
