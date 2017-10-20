#!/bin/sh

if [ "x-w" = "x$1" ] ; then
  sass --watch public/css/hw.scss:public/css/hw.css
else
  sass public/css/hw.scss:public/css/hw.css
fi
