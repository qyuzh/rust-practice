/// runs in O(n)/O(1)
pub fn distance_between_bus_stops(distance: Vec<i32>, start: i32, destination: i32) -> i32 {
    let all_distances = distance.iter().sum::<i32>();
    let min = start.min(destination) as usize;
    let max = start.max(destination) as usize;
    let half = distance[min..max].iter().sum::<i32>();
    half.min(all_distances - half)
}
