# v0.5.0 - 2020/10/09

## Features

- Remove ButtplugProtocolCreator and ButtplugProtocol derives
  - ButtplugProtocolCreator is no longer a thing, and ButtplugProtocol can
    no longer be derived.

# v0.4.0 - 2020/07/26

## Features

- Fix derives to use new thiserror errors (in buttplug 0.5).

# v0.3.0 - 2020/06/22

## Features

- Add derives for:
  - ButtplugServerMessage
  - ButtplugClientMessage
  - ButtplugProtocol
  - ButtplugProtocolProperties
  - ButtplugProtocolCreator

# v0.2.0 - 2020/04/12

## Features

- Added derives for union generation and message conversion.

# v0.1.0 - 2020/02/15

## Features

- Update dependencies
- Add proc macros for enum trait derivation

# v0.0.1 - 2019/11/03

## Features

- Implementation of ButtplugMessage trait derivation macro
