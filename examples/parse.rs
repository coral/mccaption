use mccaption::MCC;
fn main() {
    let v = MCC::from_file("examples/data/BigBuckBunny_256x144-24fps.mcc").unwrap();

    println!(
        "TC format: {}, Creator UUID: {}",
        v.header.timecode_format, v.header.uuid
    );

    println!("first row\n");

    println!("{}", v.lines[0].timecode);
    println!("{:?}", v.lines[0].data)
}
