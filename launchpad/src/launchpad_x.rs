use crate::{Launchpad, LaunchpadOutput, LaunchpadColor, LaunchpadState};
use crate::event::*;
use std::io::{stdin, stdout, Write};

use std::sync::{Arc, Mutex};

use regex::Regex;

pub struct LaunchpadX {
    input: midir::MidiInputConnection<Arc<Mutex<LaunchpadEvent>>>,
    output: midir::MidiOutputConnection,

    event: Arc<Mutex<LaunchpadEvent>>
}

impl LaunchpadX {
    const LED_LAYOUT: [[u8;9];9] = [
        [ 91, 92, 93, 94, 95, 96, 97, 98, 99 ],
        [ 81, 82, 83, 84, 85, 86, 87, 88, 89 ],
        [ 71, 72, 73, 74, 75, 76, 77, 78, 79 ],
        [ 61, 62, 63, 64, 65, 66, 67, 68, 69 ],
        [ 51, 52, 53, 54, 55, 56, 57, 58, 59 ],
        [ 41, 42, 43, 44, 45, 46, 47, 48, 49 ],
        [ 31, 32, 33, 34, 35, 36, 37, 38, 39 ],
        [ 21, 22, 23, 24, 25, 26, 27, 28, 29 ],
        [ 11, 12, 13, 14, 15, 16, 17, 18, 19 ],
    ];

    fn lookup_layout(pad: u8) -> Option<(usize, usize)>
    {
        for y in 0..9 {
            for x in 0..9 {
                if LaunchpadX::LED_LAYOUT[y][x] == pad {
                    return Some((x, y));
                }
            }
        }
        None
    }

    pub fn init() -> Result<Box<dyn Launchpad>, Box<dyn std::error::Error>> {
        let event = Arc::new(Mutex::new(LaunchpadEvent::default()));

        let input = LaunchpadX::init_input_device(event.clone())?;
        let output = LaunchpadX::init_output_device()?;

        let mut launchpad = Box::new(LaunchpadX {
            input, output, event
        });

        let message = [0xF0,0x00,0x20,0x29,0x02,0x0C,0x0E,0x01,0xF7];
        launchpad.send(&message);

        Ok(launchpad)
    }

    pub fn get_input_device<'a>(
        midi_in: &midir::MidiInput,
        in_ports: &'a midir::MidiInputPorts
    ) -> Result<&'a midir::MidiInputPort, Box<dyn std::error::Error>> {
        match in_ports.len() {
            0 => return Err("No input port found.".into()),
            1 => {
                println!("Only available port: {}", midi_in.port_name(&in_ports[0]).unwrap());
                Ok(&in_ports[0])},
            _ => {
                // if it matches the RegEx, then use that port.
                let re = Regex::new(r"Launchpad X MIDI 2").unwrap();
                let mut selected_port = None;
                for port in in_ports.iter() {
                    if re.is_match(&midi_in.port_name(port).unwrap()) {
                        selected_port = Some(port);
                    }
                }

                match selected_port {
                    None => {
                        // otherwise, prompt the user for the port.
                        println!("Available input ports:");
                        for (i, p) in in_ports.iter().enumerate() {
                            println!("{}: {}", i, midi_in.port_name(p).unwrap());
                        }
                        print!("Please select input port: ");
                        stdout().flush()?;
                        let mut input = String::new();
                        stdin().read_line(&mut input)?;
                        in_ports.get(input.trim().parse::<usize>()?)
                            .ok_or("Invalid input port selected.".into())
                    },
                    Some(port) => {
                        Ok(port)
                    }
                }
            }
        }
    }

    pub fn init_input_device(
        event: Arc<Mutex<LaunchpadEvent>>
    ) -> Result<midir::MidiInputConnection<Arc<Mutex<LaunchpadEvent>>>, Box<dyn std::error::Error>> {
        let midi_in = midir::MidiInput::new("Launchpad Output")?;
        
        let in_ports = midi_in.ports();

        let in_port: &midir::MidiInputPort = LaunchpadX::get_input_device(&midi_in, &in_ports).unwrap();

        println!("Opening connection");
        let conn_in = midi_in.connect(in_port, "launchpad-api", move |_stamp, message, event| {
            match LaunchpadX::parse_midi_message(message) {
                Some(args) => event.lock().unwrap().trigger(args),
                None => ()
            }
        }, event)?;
        println!("Connection open.");
        
        Ok(conn_in)
    }

    pub fn get_output_device<'a>(midi_out: &midir::MidiOutput, out_ports: &'a midir::MidiOutputPorts) -> Result<&'a midir::MidiOutputPort, Box<dyn std::error::Error>> {
        match out_ports.len() {
            0 => return Err("No input port found.".into()),
            1 => {
                println!("Only available port: {}", midi_out.port_name(&out_ports[0]).unwrap());
                Ok(&out_ports[0])},
            _ => {
                // if it matches the RegEx, then use that port.
                let re = Regex::new(r"Launchpad X MIDI 2").unwrap();
                let mut selected_port = None;
                for port in out_ports.iter() {
                    if re.is_match(&midi_out.port_name(port).unwrap()) {
                        selected_port = Some(port);
                    }
                }

                match selected_port {
                    None => {
                        // otherwise, prompt the user for the port.
                        println!("Available output ports:");
                        for (i, p) in out_ports.iter().enumerate() {
                            println!("{}: {}", i, midi_out.port_name(p).unwrap());
                        }
                        print!("Please select output port: ");
                        stdout().flush()?;
                        let mut input = String::new();
                        stdin().read_line(&mut input)?;
                        out_ports.get(input.trim().parse::<usize>()?)
                            .ok_or("Invalid output port selected.".into())
                    },
                    Some(port) => {
                        Ok(port)
                    }
                }
            }
        }
    }

    pub fn init_output_device() -> Result<midir::MidiOutputConnection, Box<dyn std::error::Error>> {
        let midi_out = midir::MidiOutput::new("Launchpad Output")?;
        
        let out_ports = midi_out.ports();
        let out_port: &midir::MidiOutputPort = LaunchpadX::get_output_device(&midi_out, &out_ports).unwrap();

        println!("Opening connection");
        let conn_out = midi_out.connect(out_port, "launchpad-api")?;
        println!("Connection open.");

        Ok(conn_out)
    }

    pub fn send(&mut self, message: &[u8]) {
        match self.output.send(message) {
            Ok(_) => (),
            Err(err) => println!("Error: {}", err)
        };
    }

    pub fn parse_midi_message(message: &[u8]) -> Option<LaunchpadEventArgs> {
        match message[0] {
            0x80 => {
                match LaunchpadX::lookup_layout(message[1]) {
                    Some((x, y)) => Some(LaunchpadEventArgs::Released { x, y }),
                    None => None
                }
            }
            0x90 => {
                match LaunchpadX::lookup_layout(message[1]) {
                    Some((x, y)) => match message[2] {
                        0 => Some(LaunchpadEventArgs::Released { x, y }),
                        _ => Some(LaunchpadEventArgs::Pressed { x, y })
                    },
                    None => None
                }
            }
            0xB0 => {
                match LaunchpadX::lookup_layout(message[1]) {
                    Some((x, y)) => match message[2] {
                        0 => Some(LaunchpadEventArgs::Released { x, y }),
                        _ => Some(LaunchpadEventArgs::Pressed { x, y })
                    },
                    None => None
                }
            }
            _ => None
        }
    }
}

impl LaunchpadOutput for LaunchpadX {
    fn set_light(&mut self, x: usize, y: usize, color: LaunchpadColor) {
        let message = [
            0xF0, // start a SysEx message.
            0x00, 0x20, 0x29, 0x02, 0x0C, 0x03, // header for setting LaunchpadX color.
            0x03, // use RGB color type.
            LaunchpadX::LED_LAYOUT[y][x], // index the light using x and y.
            color.red, color.green, color.blue, // set it to this color.
            0xF7 // end a SysEx message.
        ];

        self.send(&message);
    }

    fn set_all_lights(&mut self, color: LaunchpadColor) {
        let mut message = vec![
            0xF0, // start a SysEx message.
            0x00, 0x20, 0x29, 0x02, 0x0C, 0x03, // header for setting LaunchpadX color.
        ];

        for y in 0..9 {
            for x in 0..9 {
                message.push(0x03); // use RGB color type.
                message.push(LaunchpadX::LED_LAYOUT[y][x]); // index the light using x and y.
                message.append(&mut vec![color.red, color.green, color.blue]); // set it to this color.
            }
        }

        message.push(0xF7); // end a SysEx message.

        self.send(&message);
    }

    fn set_state(&mut self, lights: LaunchpadState) {
        let mut message = vec![
            0xF0, // start a SysEx message.
            0x00, 0x20, 0x29, 0x02, 0x0C, 0x03, // header for setting LaunchpadX color.
        ];

        for y in 0..9 {
            for x in 0..9 {
                let color = &lights.get_lights()[y][x];

                message.push(0x03); // use RGB color type.
                message.push(LaunchpadX::LED_LAYOUT[y][x]); // index the light using x and y.
                message.append(&mut vec![color.red, color.green, color.blue]); // set it to this color.
            }
        }

        message.push(0xF7); // end a SysEx message.

        self.send(&message);
    }

    fn set_box(&mut self, x: usize, y: usize, width: usize, height: usize, color: LaunchpadColor) {
        let mut message = vec![
            0xF0, // start a SysEx message.
            0x00, 0x20, 0x29, 0x02, 0x0C, 0x03, // header for setting LaunchpadX color.
        ];

        for y in y..y+height {
            for x in x..x+width {
                message.push(0x03); // use RGB color type.
                message.push(LaunchpadX::LED_LAYOUT[y][x]); // index the light using x and y.
                message.append(&mut vec![color.red, color.green, color.blue]); // set it to this color.
            }
        }

        message.push(0xF7); // end a SysEx message.

        self.send(&message);
    }

    fn clear_grid(&mut self) {
        self.set_all_lights(LaunchpadColor::BLACK);
    }
}

impl Launchpad for LaunchpadX {
    fn get_event(&self) -> &Arc<Mutex<LaunchpadEvent>> {
        &self.event
    }


    fn set_event_handler(&self, handler: Box<dyn LaunchpadEventHandler>) {
        &self.event.lock().unwrap().subscribe(handler);
    }
}