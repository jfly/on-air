fn webcam_streaming(_device: &str) -> bool {
    false
}

fn main() {
    let device: &str = "/dev/video5";
    let is_streaming = webcam_streaming(device);

    if is_streaming {
        println!("{device} is streaming");
    } else {
        println!("{device} is not streaming");
    }
}
