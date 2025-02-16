use midir::{MidiOutput, MidiOutputConnection, MidiOutputPort};
use std::io::{stdin, stdout, Write};

pub fn create_midi_output_connection() -> anyhow::Result<MidiOutputConnection> {
    let midi_out = MidiOutput::new("My Test Output")?;

    // Get an output port (read from console if multiple are available)
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return anyhow::bail!("no output port found"),
        1 => {
            // println!(
            //     "Choosing the only available output port: {}",
            //     midi_out.port_name(&out_ports[0])?
            // );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p)?);
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or_else(||anyhow::anyhow!("help"))?
        }
    };

    println!("\nOpening connection");
    Ok(midi_out.connect(out_port, "midir-test")?)
}
