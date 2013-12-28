#!/bin/bash
rustpkg clean calm
rustpkg build calm
rustpkg clean example
rustpkg build example
rustpkg install example
