use everything_structures::{
    Bit, BitSlot, BytesStructure, Object, Property, Structure, TextStructure,
};
use tracing_subscriber::{Registry, layer::SubscriberExt};
use tracing_tree::HierarchicalLayer;

fn main() {
    tracing::subscriber::set_global_default(Registry::default().with(HierarchicalLayer::new(2)))
        .unwrap();

    let s = Structure::new(&mut [
        Property::new_bit_slot(BitSlot::Slot0, Bit::Zero),
        Property::new_bit_slot(BitSlot::Slot1, Bit::Zero),
        Property::new_bit_slot(BitSlot::Slot2, Bit::One),
        Property::new_bit_slot(BitSlot::Slot3, Bit::Zero),
        Property::new_bit_slot(BitSlot::Slot4, Bit::One),
        Property::new_bit_slot(BitSlot::Slot5, Bit::One),
        Property::new_bit_slot(BitSlot::Slot6, Bit::One),
        Property::new_bit_slot(BitSlot::Slot7, Bit::One),
    ]);

    match s {
        Structure::Byte(byte) => println!("{:?}", byte),
        _ => println!(":/"),
    }
}
