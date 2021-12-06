# Taliesin

Command line application to trigger audio notifications based on pressed keys and timers.

## Usage
With a sfx.wav file next to your executable, to trigger that sound when pressing A or Z after a delay of 1 second, and 
allowing E or Enter to interrupt it, start the application as

`.\taliesin -k A Z -f sfx.wav -d 1000 -c E Enter`