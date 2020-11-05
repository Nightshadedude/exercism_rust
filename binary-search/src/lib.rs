pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 { return None; }
    let mut l = 0usize;
    let mut r = array.len()-1; 
    while l != r {
        let m = ((l as f64 + r as f64) / 2f64).ceil() as usize; 
        if array[m] > key { r = m - 1; }
        else { l = m; }
    }
    if array[l] == key  { return Some(l); }
    return None;  
}
