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
- help view
  - list available functions
- midi transmit
  - expectation of compatibility with Ableton

## eventualities
- file i/o
  - load, save sequence character map
- configuration view
  - configure octave range
  - midi port settings
  - load file w/ lookup
  - save file
- target WASM for browser build
- receive midi clock
  - Ableton compatibility
- configure midi transmit options
  - configure event types
- support osc
