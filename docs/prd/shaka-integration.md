# Feature PRD: Shaka Player Web Integration

## Overview

Add a [Shaka Player](https://github.com/shaka-project/shaka-player) integration as a new Layer 3 package (`plinth-shaka`). Shaka is Google's open-source adaptive streaming library for web, primarily targeting DASH but also supporting HLS and other formats. It is a common alternative to Hls.js in production streaming applications.

This integration follows the same three-layer architecture and mirrors the `plinth-hlsjs` implementation as closely as Shaka's API allows.


## Goals

- Application developers integrate in a single `await PlinthShaka.initialize(player, video, videoMeta)` call
- No changes to `plinth-core` or `plinth-js` — Layer 2 is reused as-is
- Event mapping is correct for Shaka's buffering model, which differs meaningfully from Hls.js
- Test coverage matches `plinth-hlsjs` (fake player, no real network, no Wasm)
