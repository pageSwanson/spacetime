fn init() {
    // initialize spacetime
    //  provide tick rate
    // start clock
    //  set tick rate
    //  register tick function
    // start midi
    //  read port argument for midi port
    //  attempt connect to port, err on failure
    //  start midi connection
    // initialize the term UI
}

fn destroy() {
    // destroy terminal UI
    // destroy midi connection
    // destroy clock
    // destroy spacetime
}

fn transmit_midi(midi_event) {
    // send midi events
}

fn help() {
    // draw spacetime.help content
}

fn draw(character_map) {
   // draw from character map struct
   //  including sequence steps, cursor position, tempo indicator, midi port + midi transmissions
}

fn tick(spacetime) {
    // event_details = spacetime.tick()
    // transmit_midi(midi_event_details)
    // draw(spacetime.get_character_map)
}

fn on_key_press(key) {
    // spacetime.on_key_press(key)
    //  spacetime locates the cursor within the sequence
    //  spacetime updates the sequence or updates the cursor position
}

fn main() {
    // init()
    // while (1) {
    //  check for key press
    //   if no key press, continue
    //   if kill signal, break
    //  on_key_press(key)
    // }
    // destroy()
    println!("Hello, world!");
}
