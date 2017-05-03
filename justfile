
run:
  cargo build
  xterm -fa "PressStart2p" -fs 7 -e "./target/debug/ascii-raycaster 2> log.txt"

