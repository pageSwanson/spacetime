## requirements

- command line entry point
- command line buffer graphics
- command line user interaction
  - update function indicators
  - show cursor
- main view for sequencer
  - display clock rate (bpm)
  - edit clock rate if set internally
  - display midi port number
  - indicator for midi transmit
    - show note event
- configuration view
  - midi port settings
  - load file w/ lookup
  - save file
- help view
  - list available functions
- midi transmit
  - expectation of compatibility with Ableton
- midi clock receive
  - expectation of compatibility with Ableton
  - low desire for extended midi receive
- file i/o
  - on load and save

## eventualities
- target WASM for browser build
- receive midi events
- configure midi transmit options
  - configure octave range
  - configure event types
- support osc
