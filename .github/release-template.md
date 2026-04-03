## macOS

The binaries are unsigned. To clear Apple's security warning before running:

```bash
xattr -d com.apple.quarantine corpus-macos-arm64
# or for Intel Macs:
xattr -d com.apple.quarantine corpus-macos-x86_64
```

## Changelog
