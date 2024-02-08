mod webcam;

use webcam::Webcam;

fn main() {
    for webcam in Webcam::all() {
        let is_streaming = webcam.is_streaming();
        let Webcam { path, name, .. } = webcam;
        let path = path.to_string_lossy();
        let name = name.unwrap_or("???".to_string());

        if is_streaming {
            println!("{path} ({name}) is streaming");
        } else {
            println!("{path} ({name}) is not streaming");
        }
    }
}
