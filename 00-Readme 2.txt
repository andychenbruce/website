
======================================================================

Validators:

tidy -q -o /dev/null < index.html

HTML Validator:   https://validator.w3.org/

CSS Validator: https://jigsaw.w3.org/css-validator/validator
======================================================================
.emacs

(setq-default css-indent-offset 2)

======================================================================
To resize a video:

ffmpeg -i input.mp4 scale=640:480,setsar=1:1 output.mp4

======================================================================


To convert .mov to .mp4 :

  ffmpeg -i robot.mov -vcodec h264 -acodec mp2 robot.mp4


--------

h265 is a newer and better encoding:

  ffmpeg -i robot.mov -vcodec libx265 -acodec mp2 robot.mp4

WARNING:  H265 does not work in Chrome !!!!!

--------

Use "-crf" to specify quality / size tradeoff:
A higher number means worse quality and smaller size:

  ffmpeg -i robot.mov -vcodec libx264 -crf 30 -acodec mp2 robot.mp4

======================================================================

To get the width and height on an mp4:

ffprobe -v error -show_entries stream=width,height robot.h264.30.mp4


======================================================================
