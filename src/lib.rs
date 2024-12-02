use adb_client::{ADBDeviceExt, ADBServer, ADBServerDevice};
use module::Dimensions;

mod module;
pub struct ADBControl {
    device_id: String,
    device: ADBServerDevice,
}

impl ADBControl {
    pub fn new(server: Option<ADBServer>, device_id: Option<String>) -> ADBControl {
        let mut server: ADBServer = server.unwrap_or(ADBServer::default());
        let device_id: String = device_id.unwrap_or(String::from(""));

        let device: ADBServerDevice = if device_id.is_empty() {
            server.get_device().expect("cannot get device")
        } else {
            server
                .get_device_by_name(&device_id)
                .expect("cannot get device")
        };

        ADBControl { device_id, device }
    }

    pub fn input_help(&mut self) {
        let _ = self
            .device
            .shell_command(["input", "--help"], std::io::stdout());
    }

    pub fn get_device_id(&self) -> &String {
        &self.device_id
    }

    pub fn get_device_dimensions(&mut self) -> Result<Dimensions, Box<dyn std::error::Error>> {
        let mut output_buffer = Vec::new();
        let _ = self
            .device
            .shell_command(["wm", "size"], &mut output_buffer)
            .expect("cannot get command output");
        let command_output =
            String::from_utf8(output_buffer).expect("cannot convert command output to string");

        let mut dimensions: Dimensions = Dimensions {
            width: 0,
            height: 0,
        };
        for line in command_output.split("\n") {
            if line.contains("Physical size:") {
                let parts: Vec<&str> = line.split(" ").collect();
                dimensions.width = parts[2].split("x").collect::<Vec<&str>>()[0]
                    .parse::<u32>()
                    .expect("cannot parse width");
                dimensions.height = parts[2].split("x").collect::<Vec<&str>>()[1]
                    .parse::<u32>()
                    .expect("cannot parse height");
                break;
            }
        }

        Ok(dimensions)
    }

    pub fn tap(&mut self, x: u32, y: u32) -> Result<bool, Box<dyn std::error::Error>> {
        let mut output_buffer = Vec::new();
        let _ = self
            .device
            .shell_command(
                ["input", "tap", &x.to_string(), &y.to_string()],
                &mut output_buffer,
            )
            .expect("cannot get command output");
        let command_output =
            String::from_utf8(output_buffer).expect("cannot convert command output to string");

        Ok(command_output.is_empty())
    }

    pub fn swipe(
        &mut self,
        x1: u32,
        y1: u32,
        x2: u32,
        y2: u32,
        duration: u32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut output_buffer = Vec::new();
        let _ = self
            .device
            .shell_command(
                [
                    "input",
                    "swipe",
                    &x1.to_string(),
                    &y1.to_string(),
                    &x2.to_string(),
                    &y2.to_string(),
                    &duration.to_string(),
                ],
                &mut output_buffer,
            )
            .expect("cannot get command output");
        let command_output =
            String::from_utf8(output_buffer).expect("cannot convert command output to string");

        Ok(command_output.is_empty())
    }

    pub fn long_tap(
        &mut self,
        x: u32,
        y: u32,
        duration: u32,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut output_buffer = Vec::new();
        let _ = self
            .device
            .shell_command(
                [
                    "input",
                    "swipe",
                    &x.to_string(),
                    &y.to_string(),
                    &x.to_string(),
                    &y.to_string(),
                    &duration.to_string(),
                ],
                &mut output_buffer,
            )
            .expect("cannot get command output");
        let command_output =
            String::from_utf8(output_buffer).expect("cannot convert command output to string");

        Ok(command_output.is_empty())
    }

    pub fn text(&mut self, text: String) -> Result<bool, Box<dyn std::error::Error>> {
        let mut output_buffer = Vec::new();
        let text = text.clone().replace('"', "\\\"");
        let _ = self
            .device
            .shell_command(["input", "text", text.as_str()], &mut output_buffer)
            .expect("cannot get command output");
        let command_output =
            String::from_utf8(output_buffer).expect("cannot convert command output to string");

        Ok(command_output.is_empty())
    }

    pub fn keyevent(
        &mut self,
        keycode: &str,
        longpress: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut output_buffer = Vec::new();
        let mut command_segments = vec!["input", "keyevent", keycode];
        if longpress {
            command_segments.push("--longpress");
        }
        let _ = self
            .device
            .shell_command(command_segments, &mut output_buffer)
            .expect("cannot get command output");
        let command_output =
            String::from_utf8(output_buffer).expect("cannot convert command output to string");

        Ok(command_output.is_empty())
    }

    pub fn roll(&mut self, x: u32, y: u32) -> Result<bool, Box<dyn std::error::Error>> {
        let mut output_buffer = Vec::new();
        let _ = self
            .device
            .shell_command(
                ["input", "roll", &x.to_string(), &y.to_string()],
                &mut output_buffer,
            )
            .expect("cannot get command output");
        let command_output =
            String::from_utf8(output_buffer).expect("cannot convert command output to string");

        Ok(command_output.is_empty())
    }
}
