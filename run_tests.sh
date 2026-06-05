#!/bin/bash
pytest tests/
cd tools/cil && cargo test
