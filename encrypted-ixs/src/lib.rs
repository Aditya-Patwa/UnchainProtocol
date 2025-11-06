use arcis_imports::*;

#[encrypted]
mod circuits {
    use arcis_imports::*;

    pub struct BalanceValues {
        total_tips: u64,
        unclaimed_tips: u64
    }

    pub struct TipDetails {
        total_tips: u64,
        unclaimed_tips: u64
    }

    pub struct TipAmount {
        amount: u64
    }

    #[instruction]
    pub fn init_creators_balance(input_ctxt: Enc<Shared, TipAmount>) -> Enc<Shared, BalanceValues> {
        let balance_values = BalanceValues { total_tips: 0, unclaimed_tips: 0 };
        input_ctxt.owner.from_arcis(balance_values)
    }

    #[instruction]
    pub fn tip_creator(tip_amount_ctxt: Enc<Shared, TipAmount>, tip_ctxt: Enc<Mxe, TipDetails>) -> Enc<Mxe, TipDetails> {
        // let tip_amount = amount_ctxt.to_arcis();
        let tip_amount = tip_amount_ctxt.to_arcis();
        let mut tip_details = tip_ctxt.to_arcis();
        tip_details.total_tips = tip_details.total_tips + tip_amount.amount;
        tip_details.unclaimed_tips = tip_details.unclaimed_tips + tip_amount.amount;

        tip_ctxt.owner.from_arcis(tip_details)
    }

    // pub struct InputValues {
    //     v1: u8,
    //     v2: u8,
    // }

    // #[instruction]
    // pub fn add_together(input_ctxt: Enc<Shared, InputValues>) -> Enc<Shared, u16> {
    //     let input = input_ctxt.to_arcis();
    //     let sum = input.v1 as u16 + input.v2 as u16;
    //     input_ctxt.owner.from_arcis(sum)
    // }
}