# Dispatcher cleanup follow-ups

The current status and release boundaries are documented in
[known limitations](known-limitations.md).

- [#223](https://github.com/stuartbowness/darkfactory/issues/223): Linux
  parent-death cleanup does not guarantee termination of the whole descendant
  process group. This remains open.
- [#224](https://github.com/stuartbowness/darkfactory/issues/224): invalid log
  destinations could fail after spawning a worker. Release preparation moves log
  setup before spawn and adds a regression test that verifies no worker starts.

Historical review records remain in Git history; generated logs are excluded
from the current source tree.
