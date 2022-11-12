/// Sample represents a unit of audio that is generated by a Source and consumed by a Destination
type Sample = f32;

/// Source is an object that returns sound samples of an audio source
pub trait Source {
    fn sample(&mut self) -> Sample;
}

/// Destination is an object that consumes samples and outputs them to a playback device
pub trait Destination {
    fn play(&mut self, s: Sample) -> Result<(), PlaybackErr>;
}

/// PlaybackErr is returned when a Destination fails to play a Sample
///
/// This is a simple wrapper around a string that implements Display, Debug, and Error
#[derive(Debug)]
pub struct PlaybackErr {
    pub msg: String,
}

impl PlaybackErr {
    pub fn new(message: &str) -> Self {
        Self {
            msg: message.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestSource {
        val: f32,
    }

    impl Source for TestSource {
        fn sample(&mut self) -> Sample {
            let sample = self.val;
            self.val += 1.;
            sample
        }
    }

    struct TestDest {
        samples: Vec<Sample>,
    }

    impl Destination for TestDest {
        fn play(&mut self, s: Sample) -> Result<(), PlaybackErr> {
            self.samples.push(s);
            Ok(())
        }
    }

    struct TestBadDest {}

    impl Destination for TestBadDest {
        fn play(&mut self, _: Sample) -> Result<(), PlaybackErr> {
            Err(PlaybackErr::new("boom"))
        }
    }

    // test that sources can emit samples and that destinations receive them
    #[test]
    fn test_source_destination_round_trip() {
        let mut src = TestSource { val: 0. };
        let mut dest = TestDest { samples: vec![] };
        for _ in 0..10 {
            dest.play(src.sample()).expect("no playback error");
        }
        let mut want = 0.;
        for got in dest.samples {
            assert_eq!(got, want);
            want += 1.;
        }
    }

    // test that destinations emit errors when encountered
    #[test]
    fn test_playback_err_errors() {
        let mut src = TestSource { val: 0. };
        let mut dest = TestBadDest {};
        for _ in 0..10 {
            match dest.play(src.sample()) {
                Err(pe) => assert_eq!(pe.msg, "boom"),
                _ => panic!("expected playback error"),
            }
        }
    }
}
