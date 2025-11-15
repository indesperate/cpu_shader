run:
  cargo run -r

convert: run
  ffmpeg -i output_%02d.ppm -r 60 output.mp4
  mpv output.mp4

clean:
  rm *.ppm *.mp4

alias r := run
alias c := convert
