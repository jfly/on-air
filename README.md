# on air

Detect if an attached webcam is in use and optionally publish its status to a
MQTT broker in a way Home Assistant understands.

## Usage

### Just get the current status

    $ on-air get
    device: /dev/video0
        name: Integrated RGB Camera: Integrat
        status: streaming
    device: /dev/video2
        name: Integrated RGB Camera: Integrat
        status: not streaming

### Publish the current status to MQTT broker

This connects to a MQTT broker and publishes the status to a topic.

First, it publishes a retained "config" discovery message as described by Home
Assistant here:
https://www.home-assistant.io/integrations/mqtt/#discovery-topic. The component
type is `binary_sensor`.

Next, it runs forever, polling the state of the attached webcams and publishing
to the MQTT broker if anything changes.

    $ on-air mqtt --broker mqtts://mqtt.snow.jflei.com --username jfly --password hunter2 --unique-id compy386

Run `on-air mqtt --help` to discover some optional settings, including polling interval.

## Demo

You can see this in action here: <https://www.youtube.com/watch?v=9zsV41U3B2E>. Details:

- I have [systemd user targets](https://github.com/jfly/snow/blob/1c04142f85081e6478479bd4c5e9f38cedc3ed44/pattern/desktop/default.nix#L173-L186)
  configured for the various locations my laptop can be in.
- These targets are [activated by a script](https://github.com/jfly/snow/blob/1c04142f85081e6478479bd4c5e9f38cedc3ed44/snowpkgs/autoperipherals/autoperipherals/cli.py#L55)
  that currently looks at attached monitors, but may use other heuristics in
  the future.
- When I'm in my office (the garage), a [systemd unit runs that starts the `on-air` binary](https://github.com/jfly/snow/blob/1c04142f85081e6478479bd4c5e9f38cedc3ed44/pattern/garage-status.nix#L27-L34).
- The rest is all in Home Assistant: a couple of automations ("garage on-air"
  and "garage off-air") that turn the "on air led" on/off when my laptop webcam
  appears/disappears.
- I built the on air display, which was a fun mix of woodworking and
  electronics. It has a ESP8266 running [ESPHome](https://esphome.io/). It's
  directly connected to mains electricity for power.
  Feel free to reach out if you'd like more details. It may motivate me to
  document the full build.
