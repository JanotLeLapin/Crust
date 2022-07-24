pub struct Version {
    protocol: u16,
    name: String,
}

impl Version {
    pub fn protocol(self) -> u16 {
        self.protocol
    }

    pub fn name(self) -> String {
        self.name
    }
}

impl Clone for Version {
    fn clone(&self) -> Self {
        Self {
            protocol: self.protocol,
            name: self.name.clone(),
        }
    }
}

pub fn from_protocol(protocol: u16) -> Option<Version> {
    // Latest stable release associated with the given protocol id
    let name = String::from(match protocol {
        47 => "1.8.9",
        107 => "1.9",
        108 => "1.9.1",
        109 => "1.9.2",
        110 => "1.9.4",
        210 => "1.10.2",
        315 => "1.11",
        316 => "1.11.2",
        335 => "1.12",
        338 => "1.12.1",
        340 => "1.12.2",
        393 => "1.13",
        401 => "1.13.1",
        404 => "1.13.2",
        477 => "1.14",
        480 => "1.14.1",
        485 => "1.14.2",
        490 => "1.14.3",
        498 => "1.14.4",
        573 => "1.15",
        575 => "1.15.1",
        578 => "1.15.2",
        735 => "1.16",
        736 => "1.16.1",
        751 => "1.16.2",
        753 => "1.16.3",
        754 => "1.16.5",
        755 => "1.17",
        756 => "1.17.1",
        757 => "1.18.1",
        758 => "1.18.2",
        759 => "1.19",
        _ => {
            return None;
        },
    });

    Some(Version {
        protocol,
        name,
    })
}

