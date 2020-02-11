use crate::{
    create_buttplug_protocol
};

create_buttplug_protocol!(
    // Protocol name
    Maxpro,
    // No extra members
    (),
    // Only implements VibrateCmd
    (
        (VibrateCmd, {
            // TODO Convert to using generic command manager

            // Speed range for Maxpro toys are 10-100 for some reason.
            let max_value: f64 = 100.0;
            let speed: u8 = (msg.speeds[0].speed * max_value) as u8;
            let mut data = vec![0x55, 0x04, 0x07, 0xff, 0xff, 0x3f, speed, 0x5f, speed, 0x00];
            let mut crc: u8 = 0;

            for b in data.clone() {
                crc = crc.wrapping_add(b);
            }

            data[9] = crc;

            let msg = DeviceWriteCmd::new(Endpoint::Tx, data, false);
            device.write_value(msg.into()).await?;
            Ok(ButtplugMessageUnion::Ok(messages::Ok::default()))
        })
    )
);

// TODO Write some tests! Especially with the weird operational range on this.
