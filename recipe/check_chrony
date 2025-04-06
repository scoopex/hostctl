#!/bin/bash

if [ -z "$2" ];then
  echo "$0 min_servers diff_in_sec"
fi

min_servers="${1:-3}"
max_diff="${2:-0.01}"

result="$(chronyc tracking 2>&1)"
ret="$?"
if [ "$ret" != "0" ];then
    echo "ERROR: >>>$result<<<" >&2
    exit 1
fi

chronyc -c tracking|awk -F, -v "diff_in_sec=${max_diff}" '{abs_val = ($5 >= 0) ? $5 : -$5; if (abs_val > diff_in_sec){exit 1}else{exit 0}}'
ret="$?"
if [ "$ret" != "0" ];then
   echo "ERROR: time diff to high (>$max_diff)" >&2
   chronyc tracking >&2
   exit 2
fi

chronyc -c sources|awk -F, -v "min_servers=$min_servers" '$2 ~ /\+|*/ {count=count+1}END{if (count >= min_servers){exit 0}else{exit 1}}'
ret="$?"
if [ "$ret" != "0" ];then
   echo "ERROR: not enough active servers (<$min_servers)" >&2
   chronyc sources >&2
   exit 2
fi

echo "OK"
exit 0

