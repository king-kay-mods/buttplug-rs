use super::{ButtplugDeviceResultFuture, ButtplugProtocol, ButtplugProtocolCommandHandler};
use crate::{
  core::messages::{self, ButtplugDeviceCommandMessageUnion, MessageAttributesMap},
  device::{
    protocol::{generic_command_manager::GenericCommandManager, ButtplugProtocolProperties},
    DeviceImpl,
    DeviceWriteCmd,
    Endpoint,
  },
};
use std::sync::Arc;

#[derive(ButtplugProtocolProperties)]
pub struct Youcups {
  name: String,
  message_attributes: MessageAttributesMap,
  stop_commands: Vec<ButtplugDeviceCommandMessageUnion>,
}

impl ButtplugProtocol for Youcups {
  fn new_protocol(
    name: &str,
    message_attributes: MessageAttributesMap,
  ) -> Box<dyn ButtplugProtocol> {
    let manager = GenericCommandManager::new(&message_attributes);

    Box::new(Self {
      name: name.to_owned(),
      message_attributes,
      stop_commands: manager.get_stop_commands(),
    })
  }
}

impl ButtplugProtocolCommandHandler for Youcups {
  fn handle_vibrate_cmd(
    &self,
    device: Arc<Box<dyn DeviceImpl>>,
    msg: messages::VibrateCmd,
  ) -> ButtplugDeviceResultFuture {
    // TODO Convert to using generic command manager
    let msg = DeviceWriteCmd::new(
      Endpoint::Tx,
      format!("$SYS,{}?", (msg.speeds[0].speed * 8.0) as u8)
        .as_bytes()
        .to_vec(),
      false,
    );
    let fut = device.write_value(msg);
    Box::pin(async {
      fut.await?;
      Ok(messages::Ok::default().into())
    })
  }
}

// TODO Write Tests
