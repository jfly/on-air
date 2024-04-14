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
