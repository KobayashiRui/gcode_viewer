pub struct TimeData {
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
}

impl TimeData {
    pub fn new()-> Self {
        Self{
            day: 0,
            hour: 0,
            minute: 0,
        }
    }
}

pub fn sec_to_days_hours_minutes(sec:f32)->TimeData{
    let day = 86400.0;
    let hour = 3600.0;
    let minute = 60.0;

    let dout = (sec/day).floor();
    let hout = ((sec - (dout * day)) / hour).floor();
    let mout = ((sec - (dout * day) - (hout * hour)) / minute).floor();

    return TimeData{day: dout as u32, hour: hout as u32, minute:mout as u32};
}