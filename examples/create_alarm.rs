//! Gen3reate sample alarm

fn main() {
    #[cfg(feature = "tmf642")]
    {
        use tmflib::tmf642::alarm::*;
        use tmflib::tmf642::alarm_type::AlarmType;
        use tmflib::tmf642::perceived_severity::PerceivedSeverity;

        let alarm = Alarm {
            id: Some("alarm-001".to_string()),
            alarm_type: Some(AlarmType::CommunicationsAlarm),
            perceived_severity: Some(PerceivedSeverity::Major),
            ..Default::default()
        };

        println!("Created Alarm: {:?}", alarm);
    }
}
