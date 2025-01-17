# rs-media-devices-state

Rust module to get camera/microphone state -- checks if camera/microphone is ON.

## Build

```shell
git clone https://github.com/SubconsciousCompute/rs-media-devices-state
cd rs-media-devices-state
cargo build
```

## Usage

```rs
fn main() {
	let mic = media_state::microphone_state();
	let cam = media_state::camera_state();
	println!("Microphone is: {:?}", mic);
	println!("Camera is: {:?}", cam);
}
```

## Implemented APIs

| Platform | `IsCameraOn()` | `IsMicrophoneOn()` | `Debug()` | Details                                                               |
| -------- | :------------: | :----------------: | --------- | --------------------------------------------------------------------- |
| darwin   |       ☑        |         ☑          | ☑         | Using `CoreMediaIO/CMIOHardware.h` and `CoreAudio/AudioHardware.h`API |
| linux    |       ☑        |         ☑          | ☐         |
| windows  |       ☐        |         ☐          | ☐         |                                                                       |

## Examples

Check state of your media device:

```shell
cargo run --example demo-state
```

Check which app is using your media device:

```shell
cargo run --example demo-apps
```

## Troubleshooting

List all registered cameras:

```shell
// macOS
system_profiler SPCameraDataType
```

```shell
// linux
ls /dev | grep video
```

List all registered microphones and speakers:

```shell
// macOS
system_profiler SPAudioDataType
```

```shell
// linux
ls /dev/snd | grep pcm
```

## License

MIT License.
