#!/usr/bin/env bash

#$0：start.sh
#$1：-u
#$2："user;123456"
#$3：-s
#$4："share;/mount/;yes;no;no;all;user;user"

nohup /sbin/tini -- /usr/bin/samba.sh -u $2 -s $4 &

./seafile

