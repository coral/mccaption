use mccaption::MCC;
fn main() {
    let v = MCC::from_file("examples/data/BigBuckBunny_256x144-24fps.mcc").unwrap();

    dbg!(v);
}
