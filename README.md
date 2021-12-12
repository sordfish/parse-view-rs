# parse-view-rs
parses streams from /tmp/view found on Xiaomi YI Camera based on Hi3518e Chipset

h264 grabber is able to parse the video streams
- https://github.com/alienatedsec/yi-hack-v5/blob/master/src/h264grabber/h264grabber/h264grabber.c

# Helpful links

- https://github.com/shadow-1/yi-hack-v3
- https://github.com/TheCrypt0/yi-hack-v4
- https://github.com/alienatedsec/yi-hack-v5

Technical discussion on /tmp/view
- https://github.com/shadow-1/yi-hack-v3/issues/126


# Build for armv5 
```
cross build --target armv5te-unknown-linux-musleabi% 
```