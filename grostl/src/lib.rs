// #![no_std]
extern crate digest;
extern crate generic_array;

use std::marker::PhantomData;

use digest::Digest;
use generic_array::{ArrayLength, GenericArray};
use generic_array::typenum::{
    Cmp, Compare, Greater, Less, Same,
    U256, U257, U512, U1024,
};

pub type GrostlSmall<OutputSize>
    where OutputSize: ArrayLength<u8> + Cmp<U512>,
          Compare<OutputSize, U257>: Same<Less>
    = Grostl<OutputSize, U512>;

pub type GrostlBig<OutputSize>
    where OutputSize: ArrayLength<u8> + Cmp<U512>,
          Compare<OutputSize, U256>: Same<Greater>
    = Grostl<OutputSize, U1024>;

pub struct Grostl<OutputSize, BlockSize: ArrayLength<u8>> {
    state: GenericArray<u8, BlockSize>,
    phantom: PhantomData<OutputSize>,
}

fn xor_generic_array<L: ArrayLength<u8>>(
    a1: &GenericArray<u8, L>,
    a2: &GenericArray<u8, L>,
) -> GenericArray<u8, L> {
    let mut res = GenericArray::default();
    for i in 0..L::to_usize() {
        res[i] = a1[i] ^ a2[i];
    }
    res
}

impl<OutputSize, BlockSize: ArrayLength<u8>> Grostl<OutputSize, BlockSize> {
    fn new() -> Grostl<OutputSize, BlockSize> {
        // TODO: Use correct initial state
        Grostl { state: GenericArray::default(), phantom: PhantomData }
    }

    fn compress(
        &self,
        input_block: GenericArray<u8, BlockSize>,
    ) -> GenericArray<u8, BlockSize> {
        xor_generic_array(
            &xor_generic_array(
                &self.p(xor_generic_array(&self.state, &input_block)),
                &self.q(input_block)
            ),
            &self.state,
        )
    }

    fn p(
        &self,
        input_block: GenericArray<u8, BlockSize>,
    ) -> GenericArray<u8, BlockSize> {
        GenericArray::default()
    }

    fn q(
        &self,
        input_block: GenericArray<u8, BlockSize>,
    ) -> GenericArray<u8, BlockSize> {
        GenericArray::default()
    }
}

impl<OutputSize, BlockSize: ArrayLength<u8>> Default for Grostl<OutputSize, BlockSize> {
    fn default() -> Self { Self::new() }
}

impl<OutputSize: ArrayLength<u8>, BlockSize: ArrayLength<u8>> Digest for Grostl<OutputSize, BlockSize> {
    type OutputSize = OutputSize;
    type BlockSize = BlockSize;

    fn input(&mut self, input: &[u8]) {
    }

    fn result(mut self) -> GenericArray<u8, Self::OutputSize> {
        GenericArray::default()
    }
}
