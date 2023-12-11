

## ffmpeg -r 1/5 -f concat -i list.txt -c:v libx264 -r 30 out.mp4

##==============================================================================

##  This produces a really small file:

## ffmpeg -r 6 -f concat -i list.txt -crf 30 -vframes 10 -vcodec libx264 out.mp4

##==============================================================================

# loop of zero means infinite loop

## convert -delay 50 out.mp4 -loop 0 xxx.gif

##==============================================================================

ffmpeg -r 5 -f concat -i list2.txt -crf 30 -vframes 10 -vcodec libx264 out.mp4

## -r = Frame rate in Hz
## -crf = quality.  Bigger number is lower quality and smaller.
## -vframes = Number of video frames to output

convert -delay 20 out.mp4 -loop 0 xxx.gif
##==============================================================================
