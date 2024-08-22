// The 'forexlord_web3' program.
program forexlord_web3.aleo {
    transition main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }
}
