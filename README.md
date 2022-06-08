# basedname

This is a more based version of the standard unix tool `basename`. It is intended to simply be a reimplementation of it with some extra flags for fun.

# Examples

```bash
basedname -f "malware.exe" # malware
basedname -f "malware.mp4.exe" # malware.mp4
basedname -f "malware.mp4.exe" -m 2 # malware
echo "malware.mp4" | basedname # malware
```


# Roadmap

* [ ] Add flags from original `basename` so it can be used as a drop in replacement
* [ ] Rename `cli::Args::arg_parse()` to `cli::Args::parse()`
* [ ] Add a cli flag so it gathers all extensions
* [ ] Return file extension alone
