// edition:2018

fn main() {
    let s = ::xcrate::S;
    //~^ ERROR failed to resolve: could not find `xcrate` in crate root
}
