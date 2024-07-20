// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.



pub struct WrappingU32 {
    value: u32,
}
//impl Into<WrappingU32> for u32 {
    //fn into(self) -> WrappingU32 {
        //WrappingU32{value: self}
    //}
//}
impl From<u32> for WrappingU32 {
    fn from(item: u32) -> Self {
        WrappingU32{value:item}
    }
    
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}
