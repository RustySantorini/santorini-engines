use std::time::Duration;

// fn estimated_time_left(time_left:Duration, a:usize) -> Duration {
//     Duration::from_nanos((time_left.as_nanos() / (a as u128)).try_into().unwrap())
// }

// fn etl_s(time_left:Duration) -> Duration {
//     estimated_time_left(time_left, 15)
// }

pub fn get_time(time_left:Duration) -> Duration {
    // etl_s(time_left)
    time_left / 15
} 

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_1min(){
        let dur = Duration::from_secs(60);
        assert_eq!(get_time(dur), Duration::from_secs(4));
    }

    #[test]
    fn test_3min(){
        let dur = Duration::from_secs(180);
        assert_eq!(get_time(dur), Duration::from_secs(12));
    }

    #[test]
    fn test_15min(){
        let dur = Duration::from_secs(900);
        assert_eq!(get_time(dur), Duration::from_secs(60));
    }
}