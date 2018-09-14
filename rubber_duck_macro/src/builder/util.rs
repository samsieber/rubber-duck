pub fn drain_map<I, O>(input: &mut Vec<I>, filter_map: impl Fn(&mut I) -> Option<O>) -> Vec<O> {
    let mut ret = vec![];
    let mut i = 0;
    while i != input.len() {
        if let Some(val) = filter_map(&mut input[i]) {
            ret.push(val);
            input.remove(i);
        } else {
            i += 1;
        }
    }
    ret
}
