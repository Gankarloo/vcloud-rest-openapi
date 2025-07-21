# Mirror Script Analysis Summary

## Purpose
This is a bash script designed to mirror/download VMware Cloud Director API documentation from Broadcom's developer portal and package it into a zip archive.

## Key Functionality

### 1. **Input Processing**
- Takes a single argument (destination path, auto-strips `.zip` extension)
- Extracts version number from the destination path
- Constructs base URL: `https://developer.broadcom.com/xapis/vmware-cloud-director-api/{version}`

### 2. **Documentation Mirroring** (Lines 23-38)
- Uses `wget2` to recursively download HTML documentation from `/doc/` endpoint
- Configuration:
  - Infinite recursion level
  - Converts links for offline browsing
  - Only accepts HTML files
  - Removes host directories and cuts 3 directory levels
  - Handles compression and connection retries

### 3. **Additional File Fetching**
- **About page** (Line 42): Downloads `about.html` separately
- **Schema files** (Lines 47-68): 
  - Scrapes the x-references page to extract `artifactId` and `dataCategoryId`
  - Makes API request to get download URL for `schema-files.zip`
  - Downloads the schema archive

### 4. **Post-Processing**
- **Unpacks schema files** (Lines 71-74): Extracts schema-files.zip to `doc/etc/`
- **Generates metadata** (Lines 77-81): Creates `commonRes.js` with copyright and version info
- **Creates final archive** (Lines 84-86): Packages everything into a versioned zip file
- **Cleanup** (Lines 89-91): Removes temporary files and directories

## Technical Features
- **Error handling**: `set -euo pipefail` for strict error checking
- **Debug support**: `debug_wrap()` function enables verbose output when `DEBUG` environment variable is set
- **Robust parsing**: Uses grep with Perl regex and jq for JSON parsing
- **Progressive feedback**: Echo statements provide status updates throughout execution

## Usage Pattern
```bash
./mirror path/to/version-name
# Creates: version-name.zip containing mirrored documentation
```

This script effectively creates offline, self-contained documentation packages for specific versions of the VMware Cloud Director API.
