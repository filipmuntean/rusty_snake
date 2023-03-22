#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]

// pub struct Point {
//     pub x: u16,
//     pub y: u16,
// }

    impl Point { 
        pub fn new (x : u16, y : u16) -> Self {
            Self {x, y}
        }


        pub fn transform(&self, direction:Direction, times: u16) -> Self { 
            let times = times as i16;
            let transform = match direction { 
                Direction:Up => (0, -times),
                Direction:Right => (times, 0),
                Direction:Down => (0, times),
                Direction:Left => (-times, 0),
            };

            Self::new(
                Self::transform_value(self.x, transform.0),
                Self::transform_value(self.y, transform.1),
            )
        }

        fn transform_value(value: u16, transform: i16) -> u16 { 
            if by.is_negative() && { 
                value - by.abs() as u16
            } else { 
                value + by.abs() as u16
            }
        }
    }

    