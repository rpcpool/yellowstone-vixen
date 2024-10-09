#!/bin/bash

set -e
cd "$(dirname "$0")/.."

function puts() {
  if [[ -t 1 ]]; then
    echo "$@"
  else
    sed 's/\x1b\[[^m]*m//g' <<<"$@"
  fi
}

function putsc() {
  puts $'\x1b'"[$1m""$2"$'\x1b[m'
}

function cc() {
  m="$1"
  shift 1
  cmd=(cargo "${cmdbase[@]}" --manifest-path "$m" --no-default-features "$@")

  if [[ -n "$dry_run" ]]; then
    puts "${cmd[@]}"
  else
    "${cmd[@]}"
  fi
}

function ccr() {
  cc "$@" --profile=release-lite
}

cmdbase=(check)
dry_run=''
release=''
passes=(1)

while getopts cnr2 opt; do
  case "$opt" in
    c) cmdbase=(clippy --no-deps) ;;
    n) dry_run=1 ;;
    r) release=1 ;;
    2) passes=({1..2}) ;;
  esac
done

for i in "${passes[@]}"; do
  putsc 1 ":: Pass $i"

  json="$(cargo metadata --format-version 1 --no-deps)"

  for pkg in $(jq -r '.packages | keys | join("\n")' <<<"$json"); do
    pj="$(jq -r ".packages[$pkg]" <<<"$json")"

    manifest="$(jq -r '.manifest_path' <<<"$pj")"
    features="$(jq -r '.features | keys | join("\n")' <<<"$pj")"
    targets="$(jq -r '.targets | keys | join("\n")' <<<"$pj")"

    for tgt in $targets; do
      tj="$(jq -r ".targets[$tgt]" <<<"$pj")"

      name="$(jq -r '.name' <<<"$tj")"
      kinds="$(jq -r '.kind | join("\n")' <<<"$tj")"
      types="$(jq -r '.crate_types | join("/")' <<<"$tj")"

      # Not sure why lib is like that but I don't make the rules
      case "$kinds" in
        bin) kind=bin ;;
        custom-build) continue ;;
        "$types") kind=lib ;;
        *) kind="$kinds"
      esac

      putsc 1 " => $name ($kind)"

      req_features="$(jq -r '.["required-features"] | join(",")?' <<<"$tj")"

      if [[ -n "$req_features" ]]; then
        tgt_flags=(--features "$req_features")
      else
        tgt_flags=()
      fi

      case "$kind" in
        lib) kind_flags=(--lib) ;;
        *) kind_flags=("--$kind" "$name") ;;
      esac

      cc_flags=("$manifest" "${tgt_flags[@]}" "${kind_flags[@]}")

      putsc '38;5;4' '  -> No features'
      cc "${cc_flags[@]}"

      if [[ -n "$release" ]]; then
        putsc '38;5;5' '  -> No features (release)'
        ccr "${cc_flags[@]}"
      fi

      for feat in $features; do
        [[ -n "$feat" ]] || continue

        cc_feat_flags=("${cc_flags[@]}" --features "$feat")

        putsc '38;5;4' "  -> $feat"
        cc "${cc_feat_flags[@]}"

        if [[ -n "$release" ]]; then
          putsc '38;5;5' "  -> $feat (release)"
          ccr "${cc_feat_flags[@]}"
        fi
      done
    done
  done
done
