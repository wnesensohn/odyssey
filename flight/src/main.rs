use odyssey_flight::{
    engine::{EngineController, EngineInput},
    protocol::{encode, Frame},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut engine = EngineController::default();
    engine.arm().map_err(|e| format!("arm: {e:?}"))?;
    engine
        .set_throttle(0.6)
        .map_err(|e| format!("throttle: {e:?}"))?;
    for step in 0..40 {
        let input = EngineInput {
            feed_pressure_kpa: 300.0,
            chamber_temperature_k: 500.0,
            valves_ready: true,
            ignition_confirmed: step > 1,
            stop_requested: step >= 30,
        };
        let state = engine
            .tick(input, 100)
            .map_err(|e| format!("tick: {e:?}"))?;
        println!("{} {:?} {:.3}", step * 100, state, engine.throttle());
    }
    let frame = Frame {
        kind: 1,
        sequence: 42,
        payload: vec![0x00, 0x64],
    };
    let bytes = encode(&frame).map_err(|e| format!("encode: {e:?}"))?;
    println!(
        "frame={}",
        bytes.iter().map(|b| format!("{b:02x}")).collect::<String>()
    );
    Ok(())
}
