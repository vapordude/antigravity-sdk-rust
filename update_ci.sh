#!/bin/bash
sed -i '/steps:/a \    - name: Install Protoc\n      uses: arduino/setup-protoc@v3' .github/workflows/rust.yml
