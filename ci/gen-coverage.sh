#!/bin/sh

set -eu

mkdir -p public

call_grcov() {
    output_type=$1
    output_path=$2

    # Explanation of the options below:
    # grcov coverage-profiles _build               - paths where to find .rawprof (llvm) and .gcda (gcc) files, respectively
    #       --binary-path ./_build/target/debug/   - where the Rust test binaries are located
    #       --source-dir .                         - toplevel source directory
    #       --prefix-dir ../../                    - prefix to remove from C source files, since they are relative to builddir
    #       --branch                               - compute branch coverage if possible
    #       --ignore build.rs                      - https://github.com/mozilla/grcov/issues/845
    #       --ignore '**/build/markup5ever*'       - ignore generated code from dependencies
    #       --ignore '**/build/cssparser*'         - ignore generated code from dependencies
    #       --ignore 'cargo_cache/*'               - ignore code from dependencies
    #       --output-type $output_type
    #       --output-path $output_path

    grcov coverage-profiles _build               \
          --binary-path ./_build/target/debug/   \
          --source-dir .                         \
          --prefix-dir ../../                    \
          --branch                               \
          --ignore build.rs                      \
          --ignore '**/build/markup5ever*'       \
          --ignore '**/build/cssparser*'         \
          --ignore 'cargo_cache/*'               \
          --output-type $output_type             \
          --output-path $output_path
}

call_grcov cobertura coverage.xml
call_grcov html public/coverage

# Print "Coverage: 42.42" so .gitlab-ci.yml will pick it up with a regex
grep -Eo 'line-rate="[^"]+"' coverage.xml | head -n 1 | grep -Eo '[0-9.]+' | awk '{ print "Coverage:", $1 * 100 }'
